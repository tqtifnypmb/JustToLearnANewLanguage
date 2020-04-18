
pub use crate::map::Map;
pub use crate::number::Number;

pub enum Value {
    Number(Number),
    Object(Map<String, Value>),
    String(String),
    Array(Vec<Value>),
    Boolean(bool),
    Nil,
}

mod convert;
mod accessors;
mod read;