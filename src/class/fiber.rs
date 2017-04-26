use std::convert::From;

use binding::fiber;
use types::Value;

use {Class, Object, VerifiedObject};

#[derive(Debug, PartialEq)]
pub struct Fiber {
    value: Value,
}

impl Fiber {
    /// Creates a new Fiber
    pub fn new<F>(func: F) -> Value
        where F: FnMut(Value, Argc, *const Value, Value) -> Value
    {
        Self::from(fiber::new(func))
    }
}

impl From<Value> for Fiber {
    fn from(value: Value) -> Self {
        Fiber { value: value }
    }
}

impl Object for Fiber {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}
