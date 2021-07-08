use std::fmt;
use std::convert::From;
use prometheus::Error;

#[derive(Debug)]
pub enum CollectorError {
    GenerateError,
    LabelInconsistency(usize, usize),
    ContentIO(String)
}

impl std::error::Error for CollectorError {}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GenerateError => write!(f, "Error while generating gauge"),
            Self::LabelInconsistency(expect, got) => write!(f, "Label inconsistency expect: {} got: {}", expect, got),
            Self::ContentIO(msg) => write!(f, "Error with content {}", msg)
        }
    }
}

impl From<Error> for CollectorError {
    fn from(err: Error) -> Self {
        match err {
            Error::AlreadyReg => CollectorError::GenerateError,
            Error::InconsistentCardinality { expect, got} => CollectorError::LabelInconsistency(expect, got),
            Error::Io(err) => CollectorError::ContentIO(err.to_string()),
            Error::Msg(desc) => CollectorError::ContentIO(desc),
            Error::Protobuf(err) => CollectorError::ContentIO(err.to_string())
        }
    }
}

