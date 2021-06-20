# purple_exporter

[![MIT Licensed][license-badge]][license-url]

[license-badge](https://img.shields.io/badge/License-MIT-yellow.svg)
[license-url](https://opensource.org/licenses/MIT)]

Exporting metrics from the PurpleAir API for your PurpleAir Sensor.

This is where a link to the latest relase will go.

## Table of Contents

1. [Description](#description)
2. [Technical Details](#technical-details)
  2.1 [Exposed Metrics](#exposed-metrics)
    2.1.1 [Environmental Data](#environmental-data)
    2.1.2 [Estimated Mass Concentrations](#estimated-mass-concentrations)
    2.1.3 [Particle Concentrations](#particle-concentrations)
3. [Building Purple Exporter](#building-purple-exporter)
  3.1 [Getting an API Key](#getting-an-api-key)
  3.2 [Building from Source](#building-from-source)
  3.3 [Building Docker Image](#building-docker-image)
4. [Required Configuration](#required-configuration)
  4.1 [Sensor Index](#sensor-index)
  4.2 [API Read Key](#api-read-key)
  4.3 [Request Rate](#request-rate)
  4.4 [Network Port](#network-port)
  4.5 [Environmental Adjustment](#environmental-adjustment)
  4.6 [Log Level](log-level)
5. [Running Purple Exporter](#running-purple-exporter)
  5.1 [Running from Source](#running-from-source)
  5.2 [Running from Executable](#running-from-executable)
  5.3 [Running from Docker](#running-from-docker)
6. [Testing](#testing)
7. [License](#license)
8. [References](#references)

## Description

purple_exporter is a Prometheus exporter that exports a subset of available
information from a single
[PAII Purple Air Sensor](https://www2.purpleair.com/products/purpleair-pa-ii).

This project was developed so I could pipe the information from my own PAII
sensor to a grafana chart I run in my house. In the future, I may expand the
usability of this exporter to other PA sensors, or ranges of sensors.

## Technical Details

### Exposed Metrics

Metrics are collected from all sensors for which they are available.

#### Environmental Data

- Humidity: Relative humidity in the sensor chamber, on average, 4% lower than ambient humidity
- Temperature: Temperature in the sensor chamber in Farenheit, on average 8F higher than ambient temperature
- Air Pressure: Current pressure in millibars

#### Estimated Mass Concentrations

- pm1.0: Estimated mass concentration of particles less than 1 uM in diameter. (ug/m)
- pm2.5: Estimated mass concentration of particles less than 2.5 uM in diameter. (ug/m)
- pm10.0: Estimated mass concentration of particles less than 10.0 uM in diameter. (ug/m)

#### Partical Concentrations

- 0.3um: Count of particles greater than 0.3 uM in diameter. (particles/100mL)
- 0.5um: Count of particles greater than 0.5 uM in diameter. (particles/100mL)
- 1.0um: Count of particles greater than 1.0 uM in diameter. (particles/100mL)
- 2.5um: Count of particles greater than 2.5 uM in diameter. (particles/100mL)
- 5.0um: Count of particles greater than 5.0 uM in diameter. (particles/100mL)
- 10.0um: Count of particles greater than 10.0 uM in diameter. (particles/100mL)

## Building Purple Exporter

### Getting an API Key

In order to use the exporter, one will need a PurpleAir API key. This can be
obtained by e-mailing [contact@purpleair.com](mailto:contact@purpleair.com).
Approved applications will receive a read key, and a write key. This exporter
only makes use of the read key to retrieve information from the specified
sensor.

### Building From Source

`cargo build --release`

The resulting executable will be placed in `<path_to_repo>/target/release/`.

### Building Docker Image

If running from the repository root:
`docker build -t <tag> .`

This will result in a trimmed down alpine linux image with a statically compiled executable.

## Required Configuration

The exporter is configurable using the values below, specified via environmental
variable, or command line argument.

### Sensor Index

Set with the `SENSOR_ID` environment variable, or the `-s` flag.
This value is the unique index assigned by PurpleAir to the sensor that readings
will be drawn from.

### API Read Key

Set with the `READ_KEY` environment variable, or the `-x` flag.
This is one of the two API keys PurpleAir will furnish when following the above
directions.

### Request Rate

Set with the `REQUEST_RATE` environment variable, or the `-r` flag.
This is how often to query the purple air API, in seconds. This will
default to a minimum of once every 300 seconds (5 minutes).

### Network Port

Set with the `PORT` environment variable, or the `-p` flag.
This is the network port the exporter will serve its data from.

### Environmental Adjustment

Set with the `ADJUST` environment variable, or the `-a` flag.
This flag is used to indicate whether humidity and temperature readings
should be adjusted to reflect likely ambient readings rather than readings in
the sensor housing. Sensor readings are, on average, 8F hotter and 4% drier.
Valid values are `true` or `false`, and will default to `false`.

### Log Level

Set with `RUST_LOG` to indicate which level of messages to log when running
purple_exporter. Options include: `info` or `trace`.

## Running the Exporter

### Running from Source

`cargo run -- -r rate -s sensor_index -x API_read_key -p port -a adjust`

### Running from Executable

`./purple_exporter -r rate -s sensor_index -x API_read_key -p port -a adjust`

or by setting environment variables.

### Running the Docker Image

`docker run --env-file .env <image>`

An example environment variable file is provided in `.env.example`.

## Testing

Work in Progress

## License

[LICENSE](./LICENSE)

## References

- [PurpleAir API docs](https://api.purpleair.com/#api-welcome)
- Purple_Exporter makes use of seanmonstar's [pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger) to
provide activity logging.  
