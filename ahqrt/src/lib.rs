use std::ffi::c_void;

pub mod vector;
pub mod string;
pub mod boxed;
pub mod arc;

pub unsafe trait FFISafe {}

macro_rules! ffisafe {
  ($($x:ty),+) => {
    $(
      unsafe impl FFISafe for $x {}
    )*
  }
}

ffisafe! {
  u8,
  u16,
  u32,
  u64,
  i8,
  i16,
  i32,
  i64,
  usize,
  isize,
  c_void
}

unsafe impl<T> FFISafe for *const T {}
unsafe impl<T> FFISafe for *mut T {}
