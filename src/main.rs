use prometheus_exporter::prometheus::register_gauge;
use std::net::SocketAddr;

use reqwest::header;
use reqwest::blocking::Client;
use url::Url;

#[macro_use]
extern crate clap;

/// Container for a subset of sensor readings from a Purple Air sensor
/// Currently supported are:
/// # Atmospheric
/// * `atmo_sen_a[0]` -- Humidity
/// * `atmo_sen_a[1]` -- Temperature
/// * `atmo_sen_a[2]` -- Air Pressure
/// # Particle Concentration - Sensors A and B
/// * `pm_sen_a[0]` -- Concentration of particles no larger than 1 micron in diameter
/// * `pm_sen_a[1]` -- Concentration of particles no larger than 2.5 microns in diameter
/// * `pm_sen_a[2]` -- Concentration of particles no larger than 10.0 microns in diameter
/// # Particle Count - Sensors A and B
/// * `ct_sen_a[0]` -- Count of particles no lager than 0.3 microns in diameter
/// * `ct_sen_a[1]` -- Count of particles no lager than 0.5 microns in diameter
/// * `ct_sen_a[2]` -- Count of particles no lager than 1.0 microns in diameter
/// * `ct_sen_a[3]` -- Count of particles no lager than 2.5 microns in diameter
/// * `ct_sen_a[4]` -- Count of particles no lager than 5.0 microns in diameter
/// * `ct_sen_a[5]` -- Count of particles no lager than 10.0 microns in diameter
struct Reading{
    atmo_sen_a: Vec<f32>,
    pm_sen_a: Vec<f32>,
    pm_sen_b: Vec<f32>,
    ct_sen_a: Vec<f32>,
    ct_sen_b: Vec<f32>
}

/// Returns a Reading constructed from a response from the Purple Air API
/// # Arguments
/// * `raw_resp` - Response from Purple Air API as a String
/// # Returns
/// * `Reading` - A Reading struct parsed from `raw_resp`
fn get_reading(raw_resp:String, adjust:bool) -> Reading{

    // Convert Raw Response to JSON
    let sensor_response = json::parse(&raw_resp).expect("Invalid Sensor Response").remove("sensor");

    // Assemble vectors for struct from components of the raw JSON response
    let mut atmo_sen_vec = parse_response(&sensor_response, vec![String::from("humidity_a"),String::from("temperature_a"),String::from("pressure_a")]);
    let pm_sen_a_vec = parse_response(&sensor_response, vec![String::from("pm1.0_a"), String::from("pm2.5_a"), String::from("pm10.0_a")]);
    let pm_sen_b_vec = parse_response(&sensor_response, vec![String::from("pm1.0_b"), String::from("pm2.5_b"), String::from("pm10.0_b")]);
    let ct_sen_a_vec = parse_response(&sensor_response, vec![String::from("0.3_um_count_a"), String::from("0.5_um_count_a"),
        String::from("1.0_um_count_a"), String::from("2.5_um_count_a"), String::from("5.0_um_count_a"), String::from("10.0_um_count_a")]);
    let ct_sen_b_vec = parse_response(&sensor_response, vec![String::from("0.3_um_count_b"), String::from("0.5_um_count_b"),
        String::from("1.0_um_count_b"), String::from("2.5_um_count_b"), String::from("5.0_um_count_b"), String::from("10.0_um_count_b")]);

    // Apply adjustments to temperature and humidity if requested
    if adjust {
        atmo_sen_vec[0] += 4.0;
        atmo_sen_vec[1] -= 8.0;
    }

    //Need to convert this response into a proper form
    let reading = Reading{
        atmo_sen_a: atmo_sen_vec,
        pm_sen_a: pm_sen_a_vec,
        pm_sen_b: pm_sen_b_vec,
        ct_sen_a: ct_sen_a_vec,
        ct_sen_b: ct_sen_b_vec
    };

    return reading;
}

/// Create a `Vec<f32>` from a list of 
/// # Arguments
/// * `sensor_response` - All Purple Air Sensor readings, as a JSON object
/// * `keys` - `Vec<String>` of keys in `sensor_response` to assemble into returned array
/// # Returns
/// * `Vec<f32>` - values associated with `keys` as `f32`. Values default to `-1.0` if the transformation fails.
fn parse_response(sensor_response:&json::JsonValue, keys:Vec<String>) -> Vec<f32>{
    return keys.iter().map(|index| sensor_response[index].as_f32().unwrap_or(-1.0)).collect();
}

fn main() {

    // Use Clap to setup App configuration
    let args = clap_app!(myapp => 
        (version: "0.2.0")
        (author: "Wayne Manselle <wayne@viridianforge.tech")
        (about: "Purple Air API Prometheus Exporter")
        (@arg rate: -r --rate +takes_value "How often to query Purple API (seconds, min 300)")
        (@arg sensor: -s --sensor +takes_value "Purple Air Sensor to get readings from (string)")
        (@arg readkey: -x --readkey +takes_value "API Read Key for Purple Air API (string)")
        (@arg port: -p --port +takes_value "Port for exporter to listen on (string)")
        (@arg adjust: -a --adjust +takes_value "Adjust humidity and temperature to reflect ambient air")
    ).get_matches();

    // Set up configuration items
    let port_string = args.value_of("port").unwrap_or("9184");
    let adjust_string = args.value_of("adjust").unwrap_or("false");
    let rate_string = args.value_of("rate").unwrap_or("300");
    let sensor_index = args.value_of("sensor").expect("Invalid or missing Sensor Index");
    let purple_read_key = args.value_of("readkey").expect("Invalid or missing API Read Key");

    // First Parse Request Pacing Safely
    let mut request_rate:u64 = rate_string.parse().expect("Invalid request rate.");

    // Set Ambient Adjustment Flag
    let adjust_flag:bool = adjust_string.parse().unwrap_or(false);

    // Purple asks that API requests be limited to at least once every five minutes
    if request_rate < 300 {
        print!("Invalid API Request Rate set, defaulting to 300 seconds.");
        request_rate = 300;
    }

    let update_frequency = std::time::Duration::from_secs(request_rate);

    // Build Headers
    let mut headers = header::HeaderMap::new();
    headers.insert("X-API-Key", header::HeaderValue::from_str(&purple_read_key).unwrap());

    // Build Client
    let client = Client::builder()
        .default_headers(headers)
        .build().unwrap();

    //Set up Exporter Address
    let raw_addr = vec!["0.0.0.0",port_string].join(":");
    let addr : SocketAddr = raw_addr.parse().expect("Listening Address Could Not Be Parsed.");

    // Set up Metrics

    // Atmospherics
    let humidity_gauge = register_gauge!("Humidity", "Humidity Measured by Sensor").expect("Cannot create Humidity gauge.");
    let temperature_gauge = register_gauge!("Temperature", "Temperature Measured by Sensor").expect("Cannot create Temperature gauge.");
    let pressure_gauge = register_gauge!("Pressure", "Barometric Pressure Measured by Sensor").expect("Cannot create Barometric Pressure gauge.");
    
    // Particle Concentrations
    let mass_one_a = register_gauge!("PM_1_0_A", "Sensor A Estimated Concentration of Particles Less than 1 micron in diameter (ug/m^3)").expect("Cannot create 1 Micron Concentration gauge A.");
    let mass_one_b = register_gauge!("PM_1_0_B", "Sensor B Estimated Concentration of Particles Less than 1 micron in diameter (ug/m^3)").expect("Cannot create 1 Micron Concentration gauge B.");
    let mass_two_a = register_gauge!("PM_2_0_A", "Sensor A Estimated Concentration of Particles Less than 2 microns in diameter (ug/m^3)").expect("Cannot create 2 Micron Concentration gauge A.");
    let mass_two_b = register_gauge!("PM_2_0_B", "Sensor B Estimated Concentration of Particles Less than 2 microns in diameter (ug/m^3)").expect("Cannot create 2 Micron Concentration gauge B.");
    let mass_ten_a = register_gauge!("PM_10_0_A", "Sensor A Estimated Concentration of Particles Less than 10 microns in diameter (ug/m^3)").expect("Cannot create 10 Micron Concentration gauge A.");
    let mass_ten_b = register_gauge!("PM_10_0_B", "Sensor B Estimated Concentration of Particles Less than 10 microns in diameter (ug/m^3)").expect("Cannot create 10 Micron Concentration gauge B.");

    // Particle Counts
    let count_pthree_a = register_gauge!("Count_0_3_A", "Sensor A Count of Particles less than 0.3 microns in diameter").expect("Cannot create 0.3 Micron Count gauge A.");
    let count_pthree_b = register_gauge!("Count_0_3_B", "Sensor B Count of Particles less than 0.3 microns in diameter").expect("Cannot create 0.3 Micron Count gauge B.");
    let count_pfive_a = register_gauge!("Count_0_5_A", "Sensor A Count of Particles less than 0.5 microns in diameter").expect("Cannot create 0.5 Micron Count gauge A.");
    let count_pfive_b = register_gauge!("Count_0_5_B", "Sensor B Count of Particles less than 0.5 microns in diameter").expect("Cannot create 0.5 Micron Count gauge B.");
    let count_one_a = register_gauge!("Count_1_0_A", "Sensor A Count of Particles less than 1.0 microns in diameter").expect("Cannot create 1.0 Micron Count gauge A.");
    let count_one_b = register_gauge!("Count_1_0_B", "Sensor B Count of Particles less than 1.0 microns in diameter").expect("Cannot create 1.0 Micron Count gauge B.");
    let count_twopfive_a = register_gauge!("Count_2_5_A", "Sensor A Count of Particles less than 2.5 microns in diameter").expect("Cannot create 2.5 Micron Count gauge A.");
    let count_twopfive_b = register_gauge!("Count_2_5_B", "Sensor B Count of Particles less than 2.5 microns in diameter").expect("Cannot create 2.5 Micron Count gauge B.");
    let count_five_a = register_gauge!("Count_5_0_A", "Sensor A Count of Particles less than 5.0 microns in diameter").expect("Cannot create 5.0 Micron Count gauge A.");
    let count_five_b = register_gauge!("Count_5_0_B", "Sensor B Count of Particles less than 5.0 microns in diameter").expect("Cannot create 5.0 Micron Count gauge B.");
    let count_ten_a = register_gauge!("Count_10_0_A", "Sensor A Count of Particles less than 10.0 microns in diameter").expect("Cannot create 10.0 Micron Count gauge A.");
    let count_ten_b = register_gauge!("Count_10_0_B", "Sensor B Count of Particles less than 10.0 microns in diameter").expect("Cannot create 10.0 Micron Count gauge B.");

    // Start exporter
    let purple_exporter = prometheus_exporter::start(addr).expect("Cannot start exporter.");

    loop{
        // Do the things!
        println!("Starting the loop!");

        // Build URL used in GETs
        let mut url_string = "https://api.purpleair.com/v1/sensors/".to_owned();
        url_string.push_str(&sensor_index);

        let sensor_url = Url::parse(&url_string).unwrap();

        println!("Attempting to get a reading.");

        // Get a reading        
        let raw_resp = client.get(sensor_url).send()
            .expect("Response from PurpleAPI could not be retrieved.")
            .text()
            .expect("Unable to unwrap response");

        println!("{}", raw_resp);

        let reading = get_reading(raw_resp, adjust_flag);

        // Update Atmospherics
        humidity_gauge.set(reading.atmo_sen_a[0].into());
        temperature_gauge.set(reading.atmo_sen_a[1].into());
        pressure_gauge.set(reading.atmo_sen_a[2].into());

        // Update Mass Concentrations
        mass_one_a.set(reading.pm_sen_a[0].into());
        mass_one_b.set(reading.pm_sen_b[0].into());
        mass_two_a.set(reading.pm_sen_a[1].into());
        mass_two_b.set(reading.pm_sen_b[1].into());
        mass_ten_a.set(reading.pm_sen_a[2].into());
        mass_ten_b.set(reading.pm_sen_b[2].into());

        // Update Particle Counts
        count_pthree_a.set(reading.ct_sen_a[0].into());
        count_pthree_b.set(reading.ct_sen_b[0].into());
        count_pfive_a.set(reading.ct_sen_a[1].into());
        count_pfive_b.set(reading.ct_sen_b[1].into());
        count_one_a.set(reading.ct_sen_a[2].into());
        count_one_b.set(reading.ct_sen_b[2].into());
        count_twopfive_a.set(reading.ct_sen_a[3].into());
        count_twopfive_b.set(reading.ct_sen_b[3].into());
        count_five_a.set(reading.ct_sen_a[4].into());
        count_five_b.set(reading.ct_sen_b[4].into());
        count_ten_a.set(reading.ct_sen_a[5].into());
        count_ten_b.set(reading.ct_sen_b[5].into());

        println!("Wait until next update.");
        let _gaurd = purple_exporter.wait_duration(update_frequency);
    }

}
