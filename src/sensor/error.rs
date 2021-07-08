use std::convert::From;
use std::fmt;
use sds011::Error as SDError;


#[derive(Debug)]
pub enum SensorError {
    StartupError,
    RuntimeError(String),
    MaxLapAchieved
}

impl std::error::Error for SensorError {}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensorError::StartupError => write!(f, "Error while getting sensor info"),
            SensorError::RuntimeError(err) => write!(f, "Error while collecting datas from the sensor: {}", err) ,
            SensorError::MaxLapAchieved => write!(f, "Max lap has been achieved")
        }
    }
}

// Convert
impl From<SDError> for SensorError {
    fn from(err: SDError) -> Self {
        match err {
            SDError::BadChecksum => SensorError::RuntimeError("Bad checksum".to_string()),
            SDError::EmptyDataFrame => SensorError::RuntimeError("Empty data".to_string()),
            SDError::ReadError(reason) => SensorError::RuntimeError(reason),
            SDError::TooLongWorkTime => SensorError::StartupError
        }
    }
}

impl From<Box<dyn std::error::Error>> for SensorError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        SensorError::RuntimeError(err.to_string())
    }
}