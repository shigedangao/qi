use std::convert::From;
use std::fmt;
use std::sync::mpsc::SendError;
use sds011::{
    Error as SDError,
    Message
};


#[derive(Debug)]
pub enum SensorError {
    StartupError,
    RuntimeError(String),
    SendThreadError(String),
    MaxLapAchieved
}

impl std::error::Error for SensorError {}

impl fmt::Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SensorError::StartupError => write!(f, "An error during the startup of the sensor"),
            SensorError::RuntimeError(err) => write!(f, "An error occurred while collecting datas from the senso: {}", err) ,
            SensorError::SendThreadError(err) => write!(f, "Error while sending data from collecting sensor thread: {}", err),
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

impl From<SendError<Message>> for SensorError {
    fn from(err: SendError<Message>) -> Self {
        SensorError::SendThreadError(err.to_string())
    }
}
