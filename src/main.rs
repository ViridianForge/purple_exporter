use env_logger::{
    Builder,
    Env
};

use log::info;
use prometheus_exporter::prometheus::register_gauge_vec;
use prometheus_exporter::prometheus::core::MetricVec;
use std::net::SocketAddr;

// Metrics - should be grabbed from a single API call. Calls should be limited
// to only as often as Purple Requests (1-10 Minutes)
// Make absolutely sure that requests for multiple sensors are fit into a single
// request.

// Configuration Required
// API_READ_KEY
// UPDATE_FREQUENCY (60s, 300s, 600s)
// Humidity (sensor A only)
// Temperature (sensor A only)
// Pressure (sensor A only)
// Analog_Input
// SENSOR READINGS (For A and B)
// pm1.0
// pm2.5
// pm10
// 0.3_um_count
// 0.5_um_count
// 1.0_um_count
// 2.5_um_count
// 5.0_um_count
// 10.0_um_count

// Retrieves a reading from the PurpleAPI sensor API end point
// The response from this is used to update the outputs of the exporter
fn get_reading(sensor_index:String, purple_read_key:String) -> String{
    // Get Metrics
    let apiResponse = reqwest::blocking::get(&format!("https://api.purpleair.com/v1/sensors/{}", sensor_index))
    .expect("Response from PurpleAPI could not be retrieved.")
    .text(); //Needs to be JSON

    //Need to convert this response into a proper form
    apiResponse.unwrap()
}

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    //Set up configuration items
    let update_frequency = std::time::Duration::from_secs(300);
    let sensor_index = String::from("REDACTED");
    let purple_read_key = String::from("REDACTED");


    //Set up Exporter Address
    let raw_addr = "0.0.0.0:9184";
    let addr : SocketAddr = raw_addr.parse().expect("Listening Address Could Not Be Parsed.");

    //Set up Metrics 
    let env_gauge_vec = register_gauge_vec!("Environment_Gauge_Vec", "Sensor Environmental Data", &["Humidity", "Sensor Temp", "Pressure"])
        .expect("Could not register environmental metrics as vec gauage.");
    
    let pm_gauge_vec = register_gauge_vec!("PM_Gauge_Vec", "Sensor Estimated Mass Concentrations (ug/m^3)", &["pm1.0_a", "pm2.0_a", "pm10.0_a","pm1.0_b", "pm2.0_b", "pm10.0_b"])
        .expect("Could not register mass concentration metrics as vec gauage.");
    
    let um_gauge_vec = register_gauge_vec!("UM_Gauge_Vec", "Sensor Particle Counts by Size", &["0.3_um_a", "0.5_um_a", "1.0_um_a","2.5_um_a", "5.0_um_a", "10.0_um_b", "0.3_um_b", "0.5_um_b", "1.0_um_b","2.5_um_b", "5.0_um_b", "10.0_um_b"])
        .expect("Could not register particle counts as vec gauage.");

    //Start exporter
    let purple_exporter = prometheus_exporter::start(addr).expect("Cannot start exporter.");

    loop{
        //Do the things!
        let _gaurd = purple_exporter.wait_duration(update_frequency);

        //Get a reading
        let reading = get_reading(sensor_index, purple_read_key);

        //Update Metrics
    }

}
