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
  pub struct PurpleReading{
    pub atmo_sen_a: Vec<f32>,
    pub pm_sen_a: Vec<f32>,
    pub pm_sen_b: Vec<f32>,
    pub ct_sen_a: Vec<f32>,
    pub ct_sen_b: Vec<f32>
  }

  /// Returns a Reading constructed from a response from the Purple Air API
  /// # Arguments
  /// * `raw_resp` - Response from Purple Air API as a String
  /// # Returns
  /// * A PurpleReading struct parsed from `raw_resp`
  pub fn get_reading(raw_resp:String, adjust:bool) -> PurpleReading{

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

    // Convert this response into a proper form
    let reading = PurpleReading{
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
  /// * Vec of values associated with `keys` as `f32`. Values default to `-1.0` if the transformation fails.
  fn parse_response(sensor_response:&json::JsonValue, keys:Vec<String>) -> Vec<f32>{
    return keys.iter().map(|index| sensor_response[index].as_f32().unwrap_or(-1.0)).collect();
  }