use prometheus_exporter::prometheus::{register_gauge, Gauge};
use std::net::SocketAddr;

use lazy_static::lazy_static;
use crate::purple_reading::PurpleReading;

lazy_static! {
    // Atmospherics
    static ref HUMIDITY: Gauge = register_gauge!("Humidity", "Humidity Measured at the Sensor").unwrap();
    static ref TEMPERATURE: Gauge = register_gauge!("Temperature", "Temperature Measured at the Sensor").unwrap();
    static ref PRESSURE: Gauge = register_gauge!("Pressure", "Pressure Measured at the Sensor").unwrap();

    // Particle Concentrations
    static ref MASS1A: Gauge = register_gauge!("PM_1_0_A", "Sensor A Estimated Concentration of Particles Less than 1 micron in diameter (ug/m^3)").unwrap();
    static ref MASS1B: Gauge = register_gauge!("PM_1_0_B", "Sensor B Estimated Concentration of Particles Less than 1 micron in diameter (ug/m^3)").unwrap();
    static ref MASS2A: Gauge = register_gauge!("PM_2_0_A", "Sensor A Estimated Concentration of Particles Less than 2 microns in diameter (ug/m^3)").unwrap();
    static ref MASS2B: Gauge = register_gauge!("PM_2_0_B", "Sensor B Estimated Concentration of Particles Less than 2 microns in diameter (ug/m^3)").unwrap();
    static ref MASS10A: Gauge = register_gauge!("PM_10_0_A", "Sensor A Estimated Concentration of Particles Less than 10 microns in diameter (ug/m^3)").unwrap();
    static ref MASS10B: Gauge = register_gauge!("PM_10_0_B", "Sensor B Estimated Concentration of Particles Less than 10 microns in diameter (ug/m^3)").unwrap();
    
    // Particle Counts
    static ref COUNTP3A: Gauge = register_gauge!("Count_0_3_A", "Sensor A Count of Particles less than 0.3 microns in diameter").unwrap();
    static ref COUNTP3B: Gauge = register_gauge!("Count_0_3_B", "Sensor B Count of Particles less than 0.3 microns in diameter").unwrap();
    static ref COUNTP5A: Gauge = register_gauge!("Count_0_5_A", "Sensor A Count of Particles less than 0.5 microns in diameter").unwrap();
    static ref COUNTP5B: Gauge = register_gauge!("Count_0_5_B", "Sensor B Count of Particles less than 0.5 microns in diameter").unwrap();
    static ref COUNT1A: Gauge = register_gauge!("Count_1_0_A", "Sensor A Count of Particles less than 1.0 microns in diameter").unwrap();
    static ref COUNT1B: Gauge = register_gauge!("Count_1_0_B", "Sensor B Count of Particles less than 1.0 microns in diameter").unwrap();
    static ref COUNT2P5A: Gauge = register_gauge!("Count_2_5_A", "Sensor A Count of Particles less than 2.5 microns in diameter").unwrap();
    static ref COUNT2P5B: Gauge = register_gauge!("Count_2_5_B", "Sensor B Count of Particles less than 2.5 microns in diameter").unwrap();
    static ref COUNT5A: Gauge = register_gauge!("Count_5_0_A", "Sensor A Count of Particles less than 5.0 microns in diameter").unwrap();
    static ref COUNT5B: Gauge = register_gauge!("Count_5_0_B", "Sensor B Count of Particles less than 5.0 microns in diameter").unwrap();
    static ref COUNT10A: Gauge = register_gauge!("Count_10_0_A", "Sensor A Count of Particles less than 10.0 microns in diameter").unwrap();
    static ref COUNT10B: Gauge = register_gauge!("Count_10_0_B", "Sensor B Count of Particles less than 10.0 microns in diameter").unwrap();
}

/// Creates and starts a new instance of the Purple Exporter.
/// # Arguments
/// * `port_string` - Port Purple_Exporter will listen on
/// # Returns
/// * Initialized `prometheus_exporter`
pub fn start_exporter(port_string: &str) -> prometheus_exporter::Exporter{
    //Set up Exporter Address
    let raw_addr = vec!["0.0.0.0",port_string].join(":");
    let addr : SocketAddr = raw_addr.parse().expect("Listening Address Could Not Be Parsed.");

    // Start exporter
    return prometheus_exporter::start(addr).expect("Cannot start exporter.");
}

/// Updates Purple Exporter Gauges based on data in a Purple Reading
/// # Arguments
/// * `reading` - A reading from a PurpleAir sensor to update metrics with
pub fn update_metrics(reading: PurpleReading){
    // Update Atmospherics
    HUMIDITY.set(reading.atmo_sen_a[0].into());
    TEMPERATURE.set(reading.atmo_sen_a[1].into());
    PRESSURE.set(reading.atmo_sen_a[2].into());

    // Update Mass Concentrations
    MASS1A.set(reading.pm_sen_a[0].into());
    MASS1B.set(reading.pm_sen_b[0].into());
    MASS2A.set(reading.pm_sen_a[1].into());
    MASS2B.set(reading.pm_sen_b[1].into());
    MASS10A.set(reading.pm_sen_a[2].into());
    MASS10B.set(reading.pm_sen_b[2].into());

    // Update Particle Counts
    COUNTP3A.set(reading.ct_sen_a[0].into());
    COUNTP3B.set(reading.ct_sen_b[0].into());
    COUNTP5A.set(reading.ct_sen_a[1].into());
    COUNTP5B.set(reading.ct_sen_b[1].into());
    COUNT1A.set(reading.ct_sen_a[2].into());
    COUNT1B.set(reading.ct_sen_b[2].into());
    COUNT2P5A.set(reading.ct_sen_a[3].into());
    COUNT2P5B.set(reading.ct_sen_b[3].into());
    COUNT5A.set(reading.ct_sen_a[4].into());
    COUNT5B.set(reading.ct_sen_b[4].into());
    COUNT10A.set(reading.ct_sen_a[5].into());
    COUNT10B.set(reading.ct_sen_b[5].into());
}