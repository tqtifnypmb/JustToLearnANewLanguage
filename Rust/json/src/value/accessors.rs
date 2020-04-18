use super::Value;
use crate::number::Number;
use crate::map::Map;

impl Value {
    
    pub fn is_bool(&self) -> bool {
        if let Value::Boolean(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Boolean(ref v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn is_string(&self) -> bool {
        if let Value::String(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn is_array(&self) -> bool {
        if let Value::Array(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn is_object(&self) -> bool {
        if let Value::Object(_) = self {
            true
        } else {
            false
        }
    }

    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        if let Value::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }

    pub fn is_number(&self) -> bool {
        if let Value::Number(_) = self {
            true
        } else {
            false
        }
    }

    fn as_number(&self) -> Option<&Number> {
        if let Value::Number(n) = self {
            Some(n)
        } else {
            None
        }
    }

    pub fn as_u64(&self) -> Option<i64> {
        self.as_number()?.as_i64()
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.as_number()?.as_f64()
    }
}