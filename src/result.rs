use std::num::{ParseIntError, ParseFloatError};

#[derive(Debug)]
pub enum ParseRecordError {
    Integer(ParseIntError),
    Float(ParseFloatError),
}

impl From<ParseIntError> for ParseRecordError {
    fn from(err: ParseIntError) -> Self {
        ParseRecordError::Integer(err)
    }
}

impl From<ParseFloatError> for ParseRecordError {
    fn from(err: ParseFloatError) -> Self {
        ParseRecordError::Float(err)
    }
}
