use std::{
    thread,
    thread::sleep
};
use std::time::Duration;
use sds011::SDS011;
use super::State;
use crate::error::SensorError;

// Constant
const SENSOR_PORT: &str = "/dev/ttyUSB0";
const SLEEP_DURATION: u64 = 15;
const MAX_LAP: i32 = 10;

/// Run Sensor
///     Run the sensor in a loop
///
/// # Arguments
/// * `state` - State
pub fn run_sensor(state: State) -> Result<(), SensorError> {
    info!("Starting to listen to sensor");
    if state.lap > MAX_LAP {
        return Err(SensorError::MaxLapAchieved);
    }

    match SDS011::new(SENSOR_PORT) {
        Ok(sensor) => get_data_from_sensor(sensor, state),
        Err(err) => Err(SensorError::from(err))
    }
}

/// Get Data From Sensor
///     Retrieve Data From Sensor (within a thread)
///
/// # Arguments
/// * `mut sensor`- SDS011
/// * `state` - State
fn get_data_from_sensor(mut sensor: SDS011, mut state: State) -> Result<(), SensorError> {
    let gg = state.gauges.clone();
    let handle = thread::spawn(move || {
        info!("Listening to sensor in thread");
        loop {
            match sensor.query() {
                Ok(res) => {
                    let (pm10, pm25) = &gg;
                    pm25.set(res.pm25 as f64);
                    pm10.set(res.pm10 as f64);
                },
                Err(err) => panic!("{}", SensorError::from(err).to_string())
            };

            // sleep the thread for SLEEP_DURATION
            sleep(Duration::from_secs(SLEEP_DURATION));
        }
    });

    if let Err(err) = handle.join() {
        error!("Sensor listener crashed trace: {:?}", err);
        state.lap += 1;

        run_sensor(state)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use prometheus_exporter::prometheus::core::GenericGauge;
    use crate::collector::Gauges;
    use super::*;
    
    fn create_empty_gauges() -> Gauges {
        (
            GenericGauge::new("foo", "bar").unwrap(),
            GenericGauge::new("bar", "foo").unwrap()
        )
    }

    #[test]
    fn expect_to_not_run_sensor_loop() {
        let state = State { lap: 11, gauges: create_empty_gauges() };

        let res = run_sensor(state).unwrap_err();
        assert_eq!(res, SensorError::MaxLapAchieved);
    }

    #[test]
    fn expect_to_not_return_handler() {
        let state = State { lap: 0, gauges: create_empty_gauges() };
        let res = run_sensor(state).unwrap_err();
        assert_eq!(res, SensorError::RuntimeError("No such file or directory".to_owned()));
    }
}