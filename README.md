# Cloud Connector Prometheus Exporter

This Prometheus exporter exposes all metrics which are exposed by the Cloud Connetors native API to make them consumable by Prometheus. I tend to find the internal monitoring features of the Cloud Connector quite cumbersome and I didn't like the e-mail functionality as I prefer push notifications. Also using Prometheus I have more fine grade control over sent alerts as the Cloud Connector spams my mailbox due to unavailability of tunnels for 2-3 min. 

## How to use

Download the binary and execute it. It will automatically create a toml configuration file in the same directory. Currently it is not possible to point to a config file using parameters, but that is a feature I will definitely add in the near future.

`./cc-monitor-prom`

The application will only scrape the Cloud Connector APIs when it is called. The metrics are exposed to 0.0.0.0:9185/metrics.


## Example configuration
Please keep in mind that the global port parameter is currently not evaluated. Also the version parameter in the connector is being ignored as well. This will be fixed in future releases.

```
port = 9185

[[connectors]]
version = 0
ip = '127.0.0.1'
port = 8443
username = 'Administrator'
password = 'manage'

[[connectors]]
version = 1
ip = '192.0.0.1'
port = 8443
username = 'Administrator'
password = 'manage'
```