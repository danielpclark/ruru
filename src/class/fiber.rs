use std::convert::From;

use binding::fiber;
use {Object, AnyObject};
use types::{Argc, Value};
use util;

#[derive(Debug, PartialEq)]
pub struct Fiber {
    value: Value,
}

impl Fiber {
    /// Creates a new Fiber
    pub fn new<F>(func: F) -> Self
        where F: FnMut(Value, Argc, *const Value, Value) -> Value
    {
        Self::from(fiber::new::<F>(func))
    }

    /// Resumes a fiber
    pub fn resume(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = util::create_arguments(arguments);
        let result = fiber::resume(self.value(), argc, argv.as_ptr());
        AnyObject::from(result)
    }

    /// Yield to the parent fiber
    pub fn yield_f(arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = util::create_arguments(arguments);
        let result = fiber::yield_f(argc, argv.as_ptr());
        AnyObject::from(result)
    }

    /// Get the current fiber
    pub fn current() -> Self {
        Self::from(fiber::current())
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

impl VerifiedObject for Fiber {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Class::from_existing("Fiber")
    }

    fn error_message() -> &'static str {
        "Error converting to Fiber"
    }
}
