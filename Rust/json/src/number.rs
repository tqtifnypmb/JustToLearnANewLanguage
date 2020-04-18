use std::fmt::*;
use std::convert::From;

#[derive(Copy, Clone, Debug)]
pub struct Number {
    n: N
}

#[derive(Copy, Clone, Debug)]
enum N {
    Int(i64),
    Float(f64),
}

impl Number {

    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::Int(u) => Some(u),
            N::Float(_) => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::Int(i) => Some(i as f64),
            N::Float(f) => Some(f)
        }
    }
}

impl From<i64> for Number {

    fn from(v: i64) -> Number {
        Number { n: N::Int(v) }
    }
}

impl From<f64> for Number {
    
    fn from(f: f64) -> Number {
        Number { n: N::Float(f) }
    }
}

impl Display for Number {

    fn fmt(&self, formater: &mut Formatter<'_>) -> Result {
        match self.n {
            N::Int(v) => Display::fmt(&v, formater),
            N::Float(f) => Display::fmt(&f, formater)
        }
    }
}