use prometheus_exporter::prometheus::register_gauge;
use std::net::SocketAddr;

use reqwest::header;
use reqwest::blocking::Client;
use url::Url;

use clap::{Arg, App};

// Metrics - should be grabbed from a single API call. Calls should be limited
// to only as often as Purple Requests (1-10 Minutes)
// Make absolutely sure that requests for multiple sensors are fit into a single
// request.

// Convert docs to MD
struct Reading{
    humidity: f32,
    temperature: f32,
    pressure: f32,
    pm_sen_a: Vec<f32>,
    pm_sen_b: Vec<f32>,
    ct_sen_a: Vec<f32>,
    ct_sen_b: Vec<f32>
}

// Retrieves a reading from the PurpleAPI sensor API end point
// The response from this is used to update the outputs of the exporter
fn get_reading(raw_resp:String) -> Reading{

    // Convert Raw Response to JSON
    let json_response = json::parse(&raw_resp).unwrap();

    //Need to convert this response into a proper form
    let reading = Reading{
        humidity: json_response["sensor"]["humidity_a"].as_f32().unwrap(),
        temperature: json_response["sensor"]["temperature_a"].as_f32().unwrap(),
        pressure: json_response["sensor"]["pressure_a"].as_f32().unwrap(),
        pm_sen_a: vec![json_response["sensor"]["pm1.0_a"].as_f32().unwrap(),
            json_response["sensor"]["pm2.5_a"].as_f32().unwrap(), 
            json_response["sensor"]["pm10.0_a"].as_f32().unwrap()],
        pm_sen_b: vec![json_response["sensor"]["pm1.0_b"].as_f32().unwrap(),
            json_response["sensor"]["pm2.5_b"].as_f32().unwrap(), 
            json_response["sensor"]["pm10.0_b"].as_f32().unwrap()],
        ct_sen_a: vec![json_response["sensor"]["0.3_um_count_a"].as_f32().unwrap(),
            json_response["sensor"]["0.5_um_count_a"].as_f32().unwrap(), 
            json_response["sensor"]["1.0_um_count_a"].as_f32().unwrap(), 
            json_response["sensor"]["2.5_um_count_a"].as_f32().unwrap(), 
            json_response["sensor"]["5.0_um_count_a"].as_f32().unwrap(), 
            json_response["sensor"]["10.0_um_count_a"].as_f32().unwrap()],
        ct_sen_b: vec![json_response["sensor"]["0.3_um_count_b"].as_f32().unwrap(),
            json_response["sensor"]["0.5_um_count_b"].as_f32().unwrap(), 
            json_response["sensor"]["1.0_um_count_b"].as_f32().unwrap(), 
            json_response["sensor"]["2.5_um_count_b"].as_f32().unwrap(), 
            json_response["sensor"]["5.0_um_count_b"].as_f32().unwrap(), 
            json_response["sensor"]["10.0_um_count_b"].as_f32().unwrap()],
    };

    return reading;
}

fn main() {

    // Use Clap to setup App configuration
    let args = App::new("Purple Exporter")
        .version("0.1.0")
        .author("ViridianForge <wayne@viridianforge.tech")
        .about("Purple Air API Prometheus Exporter")
        .arg(Arg::with_name("rate")
            .short("r")
            .long("rate")
            .takes_value(true)
            .help("How often to query Purple API (seconds, min 300)"))
        .arg(Arg::with_name("sensor")
            .short("s")
            .long("sensor")
            .takes_value(true)
            .help("Purple Air Sensor to get readings from (string)"))
        .arg(Arg::with_name("readkey")
            .short("x")
            .long("readkey")
            .takes_value(true)
            .help("API Read Key for Purple Air API (string)"))
        .get_matches();

    // Set up configuration items
    let rate_string = args.value_of("rate").unwrap_or("300");
    let sensor_index = args.value_of("sensor").unwrap_or("Invalid or missing Sensor Index");
    let purple_read_key = args.value_of("readkey").expect("Invalid or missing API Read Key");

    // First Parse Request Pacing Safely
    let mut request_rate:u64 = rate_string.parse().expect("Invalid request rate.");

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
    let raw_addr = "0.0.0.0:9184";
    let addr : SocketAddr = raw_addr.parse().expect("Listening Address Could Not Be Parsed.");

    //Set up Metrics

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

    //Start exporter
    let purple_exporter = prometheus_exporter::start(addr).expect("Cannot start exporter.");

    loop{
        //Do the things!
        println!("Starting the loop!");

        //Build URL used in GETs
        let mut url_string = "https://api.purpleair.com/v1/sensors/".to_owned();
        url_string.push_str(&sensor_index);

        let sensor_url = Url::parse(&url_string).unwrap();

        println!("Attempting to get a reading.");

        //Get a reading        
        let raw_resp = client.get(sensor_url).send()
            .expect("Response from PurpleAPI could not be retrieved.")
            .text()
            .expect("Unable to unwrap response");

        println!("{}", raw_resp);

        let reading = get_reading(raw_resp);

        //Update Atmospherics
        humidity_gauge.set(reading.humidity.into());
        temperature_gauge.set(reading.temperature.into());
        pressure_gauge.set(reading.pressure.into());

        //Update Mass Concentrations
        mass_one_a.set(reading.pm_sen_a[0].into());
        mass_one_b.set(reading.pm_sen_b[0].into());
        mass_two_a.set(reading.pm_sen_a[1].into());
        mass_two_b.set(reading.pm_sen_b[1].into());
        mass_ten_a.set(reading.pm_sen_a[2].into());
        mass_ten_b.set(reading.pm_sen_b[2].into());

        //Update Particle Counts
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
