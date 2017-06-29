use std::convert::From;
use std::slice;

use binding::rproc;
use types::{Value, Argc};
use util;

use {AnyObject, Class, Object, VerifiedObject};

/// `Proc` (works with `Lambda` as well)
#[derive(Debug, PartialEq)]
pub struct Proc {
    value: Value,
}

impl Proc {
    /// Calls a proc with given arguments
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use]
    /// extern crate ruru;
    ///
    /// use ruru::{Class, Object, Proc, RString};
    ///
    /// class!(Greeter);
    ///
    /// methods!(
    ///     Greeter,
    ///     itself,
    ///
    ///     fn greet_rust_with(greeting_template: Proc) -> RString {
    ///         let name = RString::new("Rust").to_any_object();
    ///         let rendered_template = greeting_template.unwrap().call(vec![name]);
    ///
    ///         rendered_template.try_convert_to::<RString>().unwrap()
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Greeter", None).define(|itself| {
    ///         itself.def_self("greet_rust_with", greet_rust_with);
    ///     });
    /// }
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// class Greeter
    ///   def self.greet_rust_with(greeting_template)
    ///     greeting_template.call('Rust')
    ///   end
    /// end
    ///
    /// greeting_template = -> (name) { "Hello, #{name}!" }
    ///
    /// Greeter.greet_rust_with(greeting_template) # => "Hello, Rust!"
    /// ```
    pub fn call(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = util::create_arguments(arguments);
        let result = rproc::call(self.value(), argc, argv.as_ptr());

        AnyObject::from(result)
    }

    /// Creates a new instance of Ruby `Proc` containing given `closure`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{RString, VM};
    /// # VM::init();
    ///
    /// let string = RString::new("Hello, World!");
    ///
    /// assert_eq!(string.to_string(), "Hello, World!".to_string());
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// str = 'Hello, World!'
    ///
    /// str == 'Hello, World!'
    /// ```
    pub fn new<F>(mut closure: F) -> Self
      where F : FnMut(Vec<AnyObject>) -> AnyObject {
        let wrapped_closure = |_arg: Value, argc: Argc, argv: *const Value, _blockarg: Value| {
            unsafe {
                let slice = slice::from_raw_parts(argv, argc as usize);
                let any_objects = slice.iter().map(|v| AnyObject::from(*v));
                closure(any_objects.collect::<Vec<_>>()).value()
            }
        };
        let result = rproc::new(wrapped_closure);
        Self::from(result)
    }
}

impl From<Value> for Proc {
    fn from(value: Value) -> Self {
        Proc { value: value }
    }
}

impl Object for Proc {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Proc {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Class::from_existing("Proc")
    }

    fn error_message() -> &'static str {
        "Error converting to Proc"
    }
}
