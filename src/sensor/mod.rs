use std::{
    thread,
    thread::sleep
};
use std::time::Duration;
use sds011::SDS011;
use crate::collector::Gauges;
use super::State;

mod error;

// Constant
const SENSOR_PORT: &str = "/dev/ttyUSB0";
const SLEEP_DURATION: u64 = 15;
const MAX_LAP: i32 = 10;

/// Run Sensor
///     Run the sensor in a loop
///
/// # Arguments
/// * `state` - State
/// * `gauges` - Option<Gauges>
pub fn run_sensor(state: State, gauges: Option<Gauges>) -> Result<(), error::SensorError> {
    info!("Starting to listen to sensor");
    if state.lap > MAX_LAP {
        return Err(error::SensorError::MaxLapAchieved);
    }

    match SDS011::new(SENSOR_PORT) {
        Ok(sensor) => get_data_from_sensor(sensor, state, gauges),
        Err(err) => Err(error::SensorError::from(err))
    }
}

/// Get Data From Sensor
///     Retrieve Data From Sensor (within a thread)
///
/// # Arguments
/// * `mut sensor`- SDS011
/// * `state` - State
/// * `gauges` - Option<Gauges>
fn get_data_from_sensor(mut sensor: SDS011, mut state: State, gauges: Option<Gauges>) -> Result<(), error::SensorError> {
    let gg = gauges.clone().expect("Expect to retrieve the gauges");
    let handle = thread::spawn(move || {
        info!("Listening to sensor in thread");
        loop {
            match sensor.query() {
                Ok(res) => {
                    let (pm10, pm25) = &gg;
                    pm25.set(res.pm25 as f64);
                    pm10.set(res.pm10 as f64);
                },
                Err(err) => panic!("{}", error::SensorError::from(err).to_string())
            };

            // sleep the thread for SLEEP_DURATION
            sleep(Duration::from_secs(SLEEP_DURATION));
        }
    });

    if let Err(err) = handle.join() {
        error!("Sensor listener crashed trace: {:?}", err);
        state.lap += 1;

        run_sensor(state, gauges)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_to_not_run_sensor_loop() {
        let state = State { lap: 11 };
        let res = run_sensor(state, None).unwrap_err();
        assert_eq!(res, error::SensorError::MaxLapAchieved);
    }

    #[test]
    fn expect_to_not_return_handler() {
        let state = State { lap: 0 };
        let res = run_sensor(state, None).unwrap_err();
        assert_eq!(res, error::SensorError::RuntimeError("No such file or directory".to_owned()));
    }
}