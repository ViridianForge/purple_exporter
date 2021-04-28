# purple_exporter

purple_exporter is a basic Prometheus exporter that exports information from
a single PAII Purple Air Sensor.

This project was developed so I could pipe the information from my own PAII
sensor to a grafana chart I run in my house. In the future, I may expand the
usability of this exporter to other PA sensors, or ranges of sensors.

## Getting the Required API Key

- A PurpleAir API Key

In order to use the exporter, one will need a PurpleAir API key. This can be
obtained by e-mailing [contact@purpleair.com](mailto:contact@purpleair.com).
Approved applications will receive a read key, and a write key. This exporter
only makes use of the read key to retrieve information from the specified
sensor.

## Running the Exporter

To start the exporter from the command line:
`cargo run -- -r rate -s sensor_index -x API_read_key`

Where `rate` is how often to query the purple air API, in seconds. This will
default to a minimum of once every 300 seconds (5 minutes).

`sensor_index` is the unique index assigned by Purple to the sensor to draw
readings from.

`API_read_key` is the Purple Air API Key mentioned above.

### Exposed Metrics

Metrics are collected from all sensors for which they are available.

#### Environmental Data

- Humidity
- Temperature
- Air Pressure

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

## References

[https://api.purpleair.com/#api-welcome](PurpleAir API docs)
