use std::convert::From;
use std::slice;

use binding::fiber;
use {Object, AnyObject, VerifiedObject, Class};
use types::{Argc, Value};
use util;

#[derive(Debug, PartialEq)]
pub struct Fiber {
    value: Value,
}

impl Fiber {
    /// Creates a new Fiber
    pub fn new<F>(mut func: F) -> Self
      where F : FnMut(Vec<AnyObject>) -> AnyObject {
      let wrapped_func = |_arg: Value, argc: Argc, argv: *const Value, _blockarg: Value| {
          unsafe {
              let slice = slice::from_raw_parts(argv, argc as usize);
              let any_objects = slice.iter().map(|v| AnyObject::from(*v));
              func(any_objects.collect::<Vec<_>>()).value()
          }
      };
        Self::from(fiber::new(wrapped_func))
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
