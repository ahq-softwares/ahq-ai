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


#[cfg(test)]
mod tests {
  use crate::{FFISafe, vector::Vector};
  
  #[repr(C)]
  #[repr(align(256))]
  #[derive(Debug)]
  pub struct AHQStruct {
    data: [u8; 5]
  }

  unsafe impl FFISafe for AHQStruct {}

  #[test]
  fn test() {
    let mut myvect: Vector<AHQStruct> = Vector::new();

    myvect.push(AHQStruct { data: [30, 20, 1, 5, 30] });
    myvect.push(AHQStruct { data: [20, 25, 42, 25, 35] });
    myvect.push(AHQStruct { data: [30, 20, 1, 5, 30] });
    myvect.push(AHQStruct { data: [20, 25, 42, 25, 35] });

    let data = &myvect[0];
    println!("{data:#?}");

    let data = &myvect[1];
    println!("{data:#?}");

    let data = &myvect[2];
    println!("{data:#?}");
  }
}