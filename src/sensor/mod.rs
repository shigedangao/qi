use sds011::SDS011;
use std::sync::mpsc;
use std::{
    thread,
    thread::sleep
};
use std::time::Duration;

mod error;

// Constant
const SENSOR_PORT: &str = "/dev/ttyUSB0";

/// Run Sensor
///
/// # Description
/// Run the sensor in a loop
pub fn run_sensor() -> Result<(), error::SensorError> {
    println!("Starting sensor...");
    match SDS011::new(SENSOR_PORT) {
        Ok(mut sensor) => {
            get_data_from_sensor(sensor);
            Ok(())
        },
        Err(err) => Err(error::SensorError::from(err))
    }
}

/// Get Data From Sensor
///
/// # Description
/// Retrieve Data From Sensor (within a thread)
///
/// # Arguments
/// * `mut sensor`- SDS011
fn get_data_from_sensor(mut sensor: SDS011) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("Listening the sensor...");
        loop {
            let res = match sensor.query() {
                Ok(res) => tx.send(res),
                Err(err) => panic!("{}", error::SensorError::from(err).to_string())
            };

            if let Err(err) = res {
                panic!("{}", error::SensorError::from(err).to_string())
            }

            // sleep the thread for a just a few 
            sleep(Duration::from_secs(5u64 * 60));
        }
    });

    let res = match rx.recv() {
        Ok(res) => println!("{:?}", res),
        Err(err) => {
            // if an error happened restart the thread
            println!("Restarting the thread...");
            run_sensor();
        }
    };
}