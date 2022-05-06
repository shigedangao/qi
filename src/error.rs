use std::{fmt, net::AddrParseError};
use sds011::Error as SDError;

#[derive(Debug, PartialEq)]
pub enum SensorError {
    StartupError,
    RuntimeError(String),
    MaxLapAchieved,
    Prometheus(String)
}

impl std::error::Error for SensorError {}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensorError::StartupError => write!(f, "Unable to get sensor information. The sensor might not be plugged"),
            SensorError::RuntimeError(err) => write!(f, "Error occurred while collecting datas from the sensor: {err}") ,
            SensorError::MaxLapAchieved => write!(f, "Maximum retried has been achieved. Restart the program"),
            SensorError::Prometheus(msg) => write!(f, "Error occurred with prometheus exporter {msg}")
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

impl From<prometheus_exporter::prometheus::Error> for SensorError {
    fn from(err: prometheus_exporter::prometheus::Error) -> Self {
        SensorError::Prometheus(err.to_string())
    }
}

impl From<prometheus_exporter::Error> for SensorError {
    fn from(err: prometheus_exporter::Error) -> Self {
        SensorError::Prometheus(err.to_string())
    }
}

impl From<AddrParseError> for SensorError {
    fn from(err: AddrParseError) -> Self {
        SensorError::Prometheus(err.to_string())
    }
}