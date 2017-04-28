use ruby_sys::fiber;
use ruby_sys::util;

use types::{Argc, Value};

pub fn new<F>(func: F) -> Value
    where F: FnMut(Value, Argc, *const Value, Value) -> Value
{
    let fn_box = util::block_box_ptr_create(func);
    unsafe { fiber::rb_fiber_new(util::rbsys_block_box_ptr_value, fn_box) }
}

pub fn resume(fiber: Value, argc: Argc, argv: *const Value) -> Value {
    unsafe { fiber::rb_fiber_resume(fiber, argc, argv) }
}

pub fn yield_f(argc: Argc, argv: *const Value) -> Value {
    unsafe { fiber::rb_fiber_yield(argc, argv) }
}

pub fn current() -> Value {
    unsafe { fiber::rb_fiber_current() }
}
