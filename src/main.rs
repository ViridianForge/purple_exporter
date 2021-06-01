extern crate pretty_env_logger;
#[macro_use] extern crate log;

use reqwest::header;
use reqwest::blocking::Client;
use url::Url;

mod config;
mod purple_exporter;
mod purple_reading;

#[macro_use]
extern crate clap;

fn main() {

    let config = config::load_config();

    let update_frequency = std::time::Duration::from_secs(config.query_rate);

    pretty_env_logger::init();

    // Build Headers
    let mut headers = header::HeaderMap::new();
    headers.insert("X-API-Key", header::HeaderValue::from_str(&config.read_key).unwrap());

    // Build Client
    let client = Client::builder()
        .default_headers(headers)
        .build().unwrap();

    // Start exporter
    let purple_exporter = purple_exporter::start_exporter(&config.port);

    // Build URL used in GETs
    let url_string = "https://api.purpleair.com/v1/sensors/".to_owned() + &config.sensor_id;

    loop{
        info!("Requesting update for Sensor.");

        // Create URL from URL string - consumed by loop
        let sensor_url = Url::parse(&url_string).unwrap();

        // Request Reading from PurpleAPI server       
        let raw_resp = client.get(sensor_url).send()
            .expect("Response from PurpleAPI could not be retrieved.")
            .text()
            .expect("Unable to unwrap PurpleAPI response");

        //TODO -- Add ability to recognize and adapt to API server not responding

        info!("{}", raw_resp);

        purple_exporter::update_metrics(purple_reading::get_reading(raw_resp, config.adjust));

        let _gaurd = purple_exporter.wait_duration(update_frequency);
    }
}