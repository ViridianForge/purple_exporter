use env_logger::{
    Builder,
    Env
};

use log::info;
use prometheus_exporter::prometheus::register_gauge;
use std::net::SocketAddr;

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    //Set up Address
    let raw_addr = "0.0.0.0:9184";
    let addr : SocketAddr = raw_addr.parse().expect("Listening Address Could Not Be Parsed.");

    //Set up fake metric
    let metric = register_gauge!("simple_the_answer", "to_everything")
        .expect("Could not register 'simple_the_answer' as gauge.");

    metric.set(42.0);

    //Start exporter
    prometheus_exporter::start(addr).expect("Cannot start exporter.");

    // Get Metrics
    let body = reqwest::blocking::get(&format!("http://{}/metrics", raw_addr))
        .expect("Cannot retrieve metrics from exporter")
        .text()
        .expect("Cannot get body text from request");

    info!("Exporter metrics:\n{}", body);
}
