# purple_exporter

purple_exporter is a basic Prometheus exporter that exports information from
a single PAII Purple Air Sensor.

This project was developed so I could pipe the information from my own PAII
sensor to a grafana chart I run in my house. In the future, I may expand the
usability of this exporter to other PA sensors, or ranges of sensors.

## Getting Started

### What you'll need
- A PurpleAir API Key

Get a PurpleAir API key

Configure environment variables

## Use

Launch the thing

Use the data

https://api.purpleair.com/#api-welcome

## Sample Response
{
  "api_version" : "V1.0.6-0.0.9",
  "time_stamp" : 1617597434,
  "sensor" : {
    "sensor_index" : REDACTED,
    "name" : "REDACTED",
    "model" : "PA-II",
    "location_type" : 0,
    "latitude" : REDACTED,
    "longitude" : REDACTED,
    "altitude" : REDACTED,
    "last_seen" : 1617597338,
    "last_modified" : 1612417482,
    "private" : 0,
    "channel_state" : 3,
    "channel_flags_manual" : 0,
    "humidity_a" : 44,
    "temperature_a" : 48.0,
    "pressure_a" : 1013.42,
    "pm1.0_a" : 0.77,
    "pm1.0_b" : 0.64,
    "pm2.5_a" : 1.71,
    "pm2.5_b" : 1.05,
    "pm10.0_a" : 2.71,
    "pm10.0_b" : 1.32,
    "0.3_um_count_a" : 431.79,
    "0.3_um_count_b" : 473.08,
    "0.5_um_count_a" : 123.86,
    "0.5_um_count_b" : 120.76,
    "1.0_um_count_a" : 21.04,
    "1.0_um_count_b" : 12.14,
    "2.5_um_count_a" : 2.43,
    "2.5_um_count_b" : 1.37,
    "5.0_um_count_a" : 1.5,
    "5.0_um_count_b" : 0.22,
    "10.0_um_count_a" : 1.11,
    "10.0_um_count_b" : 0.0,
    "stats_a" : {"pm2.5":1.71,"pm2.5_10minute":0.54,"pm2.5_30minute":0.57,"pm2.5_60minute":0.98,"pm2.5_6hour":6.6,"pm2.5_24hour":9.98,"pm2.5_1week":7.98,"time_stamp":1617597338},
    "stats_b" : {"pm2.5":1.05,"pm2.5_10minute":0.56,"pm2.5_30minute":0.57,"pm2.5_60minute":0.86,"pm2.5_6hour":5.82,"pm2.5_24hour":9.0,"pm2.5_1week":7.29,"time_stamp":1617597338},
    "analog_input" : 0.05,
    "primary_id_a" : REDACTED,
    "primary_key_a" : "REDACTED",
    "secondary_id_a" : REDACTED,
    "secondary_key_a" : "REDACTED",
    "primary_id_b" : REDACTED,
    "primary_key_b" : "REDACTED",
    "secondary_id_b" : REDACTED,
    "secondary_key_b" : "REDACTED",
    "hardware" : "2.0+BME280+PMSX003-B+PMSX003-A",
    "led_brightness" : 15.0,
    "firmware_upgrade" : "6.01",
    "firmware_version" : "6.01",
    "rssi" : -84.0,
    "icon" : 0,
    "confidence_manual" : 100,
    "confidence_auto" : 100,
    "channel_flags_auto" : 0
  }
}
