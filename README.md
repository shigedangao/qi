## Qi - 氣

[![Rust](https://github.com/shigedangao/qi/actions/workflows/ci.yaml/badge.svg)](https://github.com/shigedangao/qi/actions/workflows/ci.yaml)

Just a small binary to capture air quality by using the SDS011 sensor. Datas are then expose to Prometheus which will query the binary

## Features

Just scrap pm2.5 & pm10 value and expose it to Prometheus

## Requirements

- Rust
- Raspberry pi
- SDS011 sensor
- Prometheus
- Grafana

## Install prometheus & Grafana

For prometheus follow this [guide](https://pimylifeup.com/raspberry-pi-prometheus/)
For grafana follow this [guide](https://grafana.com/tutorials/install-grafana-on-raspberry-pi/)

## Install

Make the ```qi.sh``` executable with the command ```chmod +x qi.sh```

- Copy the ```qi.service``` to the folder ```/usr/local/lib/systemd/system```
- Copy the ```qi.sh``` to the folder ```/usr/local/bin```

## Run

Run the command ```sudo systemctl start qi``` to start  