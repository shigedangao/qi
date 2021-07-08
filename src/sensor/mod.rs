use sds011::SDS011;
use std::sync::{Arc, Mutex};
use std::{
    thread,
    thread::sleep
};
use std::time::Duration;

mod error;

// Constant
const SENSOR_PORT: &str = "/dev/ttyUSB0";
const SLEEP_DURATION: u64 = 60;
const MAX_LAP: i32 = 10;

/// Run Sensor
///
/// # Description
/// Run the sensor in a loop
pub fn run_sensor(lap: Arc<Mutex<i32>>) -> Result<(), error::SensorError> {
    println!("Starting sensor...");

    // @TODO consider if we should use a lock or just pass a mut struct
    if let Ok(guard) = lap.lock() {
        if *guard > MAX_LAP {
            return Err(error::SensorError::MaxLapAchieved);
        }
    }

    match SDS011::new(SENSOR_PORT) {
        Ok(sensor) => get_data_from_sensor(sensor, lap),
        Err(err) => Err(error::SensorError::from(err))
    }
}

// @TODO Included in test unit maybe ?
// fn dummy_handler(lap: Arc<Mutex<i32>>) -> Result<(), error::SensorError> {
//     let handle = thread::spawn(move || {
//         println!("Listening the sensor...");
//         loop {
//             // sleep the thread for a just a few 
//             println!("la");
//             sleep(Duration::from_secs(1));
//             panic!("lol");
//         }
//     });

//     if let Err(err) = handle.join() {
//         // re-run the thread if limit has not been achieved
//         println!("well {:?}", err);
//         if let Ok(mut guard) = lap.lock() {
//             *guard += 1;
//         }

//         run_sensor(lap)?;
//     }

//     Ok(())
//}

/// Get Data From Sensor
///
/// # Description
/// Retrieve Data From Sensor (within a thread)
///
/// # Arguments
/// * `mut sensor`- SDS011
/// * `lap` - Arc<Mutex<i32>>
fn get_data_from_sensor(mut sensor: SDS011, lap: Arc<Mutex<i32>>) -> Result<(), error::SensorError> {
    let handle = thread::spawn(move || {
        println!("Listening the sensor...");
        loop {
            match sensor.query() {
                Ok(res) => println!("{:?}", res),
                Err(err) => panic!("{}", error::SensorError::from(err).to_string())
            };

            // sleep the thread for a just a few 
            sleep(Duration::from_secs(SLEEP_DURATION));
        }
    });

    if let Err(err) = handle.join() {
        // re-run the thread if limit has not been achieved
        println!("{:?}", err);
        if let Ok(mut guard) = lap.lock() {
            *guard += 1;
        }

        run_sensor(lap)?;
    }

    Ok(())
}