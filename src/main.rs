use prometheus_exporter::prometheus::{register_gauge};
use std::net::SocketAddr;

use reqwest::header;
use reqwest::blocking::Client;
use url::Url;

mod purple_reading;
use purple_reading::get_reading;

#[macro_use]
extern crate clap;

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
