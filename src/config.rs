use std::env;

/// Docs go here
pub struct Config {
  pub sensor_id: String,
  pub read_key: String,
  pub query_rate: u64,
  pub port: String,
  pub adjust: bool
}

/// Docs go here
pub fn load_config() -> Config{
  let env_var_not_set = env::var("SENSOR_ID").is_err();
  if env_var_not_set{
    return config_from_args();
  } else {
    return config_from_env();
  }
}

/// Docs go here
fn config_from_env() -> Config{
  // Required Environment Variables
  let sensor_id = env::var("SENSOR_ID").unwrap();
  let read_key = env::var("READ_KEY").unwrap();
  // Optional Environment Variables
  let query_rate = env::var("QUERY_RATE").unwrap_or(String::from("300"));
  let port = env::var("PORT").unwrap_or(String::from("9184"));
  let adjust = env::var("ADJUST").unwrap_or(String::from("false"));

  // Convert Query Rate to u64, adjust to bool
  let query_rate_conv = query_rate.parse::<u64>().unwrap_or(300);
  let adjust_conv = adjust.parse::<bool>().unwrap_or(false);

  let config = Config{
    sensor_id: sensor_id,
    read_key: read_key,
    query_rate: query_rate_conv,
    port: port,
    adjust: adjust_conv
  };

  return config;
}

/// Docs go here
fn config_from_args() -> Config{
  let args = clap_app!(myapp => 
    (version: "0.6.0")
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
  let adjust_flag = value_t!(args, "adjust", bool).unwrap_or(false);
  let mut request_rate = value_t!(args, "rate", u64).unwrap_or(300);
  let sensor_index = args.value_of("sensor").expect("Invalid or missing Sensor Index");
  let purple_read_key = args.value_of("readkey").expect("Invalid or missing API Read Key");

  // Purple asks that API requests be limited to at least once every five minutes
  if request_rate < 300 {
    print!("Invalid API Request Rate set, defaulting to 300 seconds.");
    request_rate = 300;
  }

  let config = Config{
    sensor_id: sensor_index.to_string(),
    read_key: purple_read_key.to_string(),
    query_rate: request_rate,
    port: port_string.to_string(),
    adjust: adjust_flag
  };

  return config;
}