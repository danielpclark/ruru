use std::convert::From;

use binding::thread;
use types::Value;

#[cfg(unix)]
use types::RawFd;

use {Class, Object, VerifiedObject};

/// `Thread`
#[derive(Debug, PartialEq)]
pub struct Thread {
    value: Value,
}

impl Thread {
    /// Creates a new green thread.
    ///
    /// The returning value of the closure will be available as `#value` of the thread
    ///
    /// # Examples
    ///
    /// ```
    /// use ruru::{Fixnum, Thread, VM};
    /// # VM::init();
    ///
    /// Thread::new(|| {
    ///     let computation_result = 1 + 2;
    ///
    ///     Fixnum::new(computation_result)
    /// });
    /// ```
    ///
    /// Ruby:
    ///
    /// ```ruby
    /// Thread.new do
    ///   computation_result = 1 + 2
    ///
    ///   computation_result
    /// end
    /// ```
    pub fn new<F, R>(func: F) -> Self
    where
        F: 'static + FnOnce() -> R,
        R: Object,
    {
        Self::from(thread::create(func))
    }

    /// Tells scheduler to switch to other threads while current thread is waiting for a
    /// readable event on the given file descriptor.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::os::unix::io::AsRawFd;
    /// use std::os::unix::net::UnixStream;
    ///
    /// use ruru::{Thread, VM};
    /// # VM::init();
    ///
    /// let (unix_socket, _) = UnixStream::pair().unwrap();
    ///
    /// Thread::wait_fd(unix_socket.as_raw_fd());
    /// ```
    #[cfg(unix)]
    pub fn wait_fd(fd: RawFd) {
        thread::wait_fd(fd);
    }

    /// Release GVL for current thread.
    ///
    /// **Warning!** Due to MRI limitations, interaction with Ruby objects is not allowed while
    /// GVL is released, it may cause unexpected behaviour.
    /// [Read more at Ruby documentation](https://github.com/ruby/ruby/blob/2fc5210f31ad23463d7b0a0e36bcfbeee7b41b3e/thread.c#L1314-L1398)
    ///
    /// You should extract all the information from Ruby world before invoking
    /// `thread_call_without_gvl`.
    ///
    /// GVL will be re-acquired when the closure is finished.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// #[macro_use] extern crate ruru;
    ///
    /// use ruru::{Class, Fixnum, Object, Thread};
    ///
    /// class!(Calculator);
    ///
    /// methods!(
    ///     Calculator,
    ///     itself,
    ///
    ///     fn heavy_computation() -> Fixnum {
    ///         let computation = || { 2 * 2 };
    ///         let unblocking_function = || {};
    ///
    ///         // release GVL for current thread until `computation` is completed
    ///         let result = Thread::call_without_gvl(
    ///             computation,
    ///             Some(unblocking_function)
    ///         );
    ///
    ///         // GVL is re-acquired, we can interact with Ruby-world
    ///         Fixnum::new(result)
    ///     }
    /// );
    ///
    /// fn main() {
    ///     Class::new("Calculator", None).define(|itself| {
    ///         itself.def("heavy_computation", heavy_computation);
    ///     });
    /// }
    /// ```
    pub fn call_without_gvl<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where
        F: 'static + FnOnce() -> R,
        G: 'static + FnOnce(),
    {
        thread::call_without_gvl(func, unblock_func)
    }

    pub fn call_without_gvl2<F, R, G>(func: F, unblock_func: Option<G>) -> R
    where
        F: 'static + FnOnce() -> R,
        G: 'static + FnOnce(),
    {
        thread::call_without_gvl2(func, unblock_func)
    }

    pub fn call_with_gvl<F, R>(func: F) -> R
    where
        F: 'static + FnOnce() -> R,
    {
        thread::call_with_gvl(func)
    }
}

impl From<Value> for Thread {
    fn from(value: Value) -> Self {
        Thread { value: value }
    }
}

impl Object for Thread {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Thread {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        object.class() == Class::from_existing("Thread")
    }

    fn error_message() -> &'static str {
        "Error converting to Thread"
    }
}
