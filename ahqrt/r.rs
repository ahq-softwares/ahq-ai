#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use libc::{
    c_char, c_int, int8_t, int16_t, int32_t, int64_t, size_t, uint8_t, uint16_t,
    uint32_t, uint64_t,
};
pub mod vector {
    use libc::size_t;
    use crate::FFISafe;
    #[repr(C)]
    /// Please make sure that the type given is a repr(C) type
    /// The vector struct is based on this assumption
    pub struct Vector<T: FFISafe> {
        ptr: *mut T,
        len: size_t,
        cap: size_t,
    }
    unsafe impl<T: FFISafe> FFISafe for Vector<T> {}
    impl<T: FFISafe> Vector<T> {}
}
pub unsafe trait FFISafe {}
unsafe impl FFISafe for u8 {}
unsafe impl FFISafe for u16 {}
unsafe impl FFISafe for u32 {}
unsafe impl FFISafe for u64 {}
unsafe impl FFISafe for i8 {}
unsafe impl FFISafe for i16 {}
unsafe impl FFISafe for i32 {}
unsafe impl FFISafe for i64 {}
unsafe impl FFISafe for c_char {}
unsafe impl FFISafe for c_int {}
unsafe impl FFISafe for size_t {}
