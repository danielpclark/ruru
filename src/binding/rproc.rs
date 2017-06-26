use ruby_sys::{rproc, util};

use binding::global::RubySpecialConsts;
use types::{Argc, InternalValue, Value};

pub fn call(rproc: Value, argc: Argc, argv: *const Value) -> Value {
    unsafe {
        rproc::rb_proc_call_with_block(rproc,
                                       argc,
                                       argv,
                                       Value::from(RubySpecialConsts::Nil as InternalValue))
    }
}

pub fn new<F>(closure: F) -> Value
    where F: FnMut(Value, Argc, *const Value, Value) -> Value {
    unsafe {
        let closure_box = util::block_box_ptr_create(closure);
        rproc::rb_proc_new(util::rbsys_block_box_ptr_value, closure_box)
    }
}
