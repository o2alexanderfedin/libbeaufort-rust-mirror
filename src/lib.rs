#![allow(unused_imports, dead_code)]

mod decrypt;
mod encrypt;
mod tableau;

pub(crate) type DarwinSizeT = u64;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32)
    -> bool;
    fn malloc(__size: u64)
    -> *mut ();
    fn calloc(__count: u64, __size: u64)
    -> *mut ();
}
