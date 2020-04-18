use std::str::FromStr;
use std::convert::From;
use crate::error::Error;
use super::Value;

impl From<&str> for Value {

    fn from(s: &str) -> Value {
        match s.parse() {
            Result::Ok(value) => value,
            Result::Err(_) => Value::Nil
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        for byte in bytes {
            
        }
        Ok(Value::Boolean(false))
    }
}