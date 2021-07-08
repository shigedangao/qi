use std::sync::{Arc, Mutex};
use std::{
    thread,
    thread::sleep
};
use std::time::Duration;
use sds011::SDS011;
use prometheus::Gauge;

mod error;

// Constant
const SENSOR_PORT: &str = "/dev/ttyUSB0";
const SLEEP_DURATION: u64 = 30;
const MAX_LAP: i32 = 10;

/// Run Sensor
///
/// # Description
/// Run the sensor in a loop
pub fn run_sensor(lap: Arc<Mutex<i32>>, gauges: &(Gauge, Gauge)) -> Result<(), error::SensorError> {
    debug!("Starting to listen to sensor");
    if let Ok(guard) = lap.lock() {
        if *guard > MAX_LAP {
            return Err(error::SensorError::MaxLapAchieved);
        }
    }

    match SDS011::new(SENSOR_PORT) {
        Ok(sensor) => get_data_from_sensor(sensor, lap, &gauges),
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
/// * `lap` - Arc<Mutex<i32>>
fn get_data_from_sensor(mut sensor: SDS011, lap: Arc<Mutex<i32>>, gauges: &(Gauge, Gauge)) -> Result<(), error::SensorError> {
    let gauges_clone = gauges.clone();
    let handle = thread::spawn(move || {
        debug!("Listening to sensor in thread");
        loop {
            match sensor.query() {
                Ok(res) => {
                    let (pm25, pm10) = gauges_clone.clone();
                    pm25.set(res.pm25 as f64);
                    pm10.set(res.pm10 as f64);
                },
                Err(err) => panic!("{}", error::SensorError::from(err).to_string())
            };

            // sleep the thread for 30s
            sleep(Duration::from_secs(SLEEP_DURATION));
        }
    });

    if let Err(err) = handle.join() {
        error!("Sensor listener crashed trace: {:?}", err);
        if let Ok(mut guard) = lap.lock() {
            *guard += 1;
        }

        run_sensor(lap, gauges)?;
    }

    Ok(())
}