#[cfg(windows)]
pub mod win32;

use std::{ffi::c_void, marker::PhantomData, ops::Deref, ptr, sync::Arc};

#[cfg(windows)]
use win32::RawMutex;

use crate::FFISafe;

#[repr(C)]
pub enum Maybe {
  Yes(*mut c_void),
  No
}

#[repr(C)]
/// Please note that the Arc may not be like you think
/// You need to make sure that you've not overused the Arc
/// 
/// You must also be aware that this Arc might be destroyed at any moment!
/// 
/// This data type uses the rust allocator as mutual ownership is not possible to implement for
/// this structure
/// 
/// The owner of this Arc has the power to completely dealloc this Arc at any moment
/// If you do not dealloc, it is leaked
pub struct Arced {
  _inner: *const c_void,
  _use: extern "C" fn(ptr: *const c_void),
  _unuse: extern "C" fn(ptr: *const c_void)
}

extern "C" fn _use_arc<T>(ptr: *const c_void) {
  unsafe { Arc::increment_strong_count(ptr) };
}

extern "C" fn _uunuse_arc<T>(ptr: *const c_void) {
  unsafe { Arc::decrement_strong_count(ptr) };
}

impl Arced {
  /// Returns the arc along with a free function that can be called to free it directly
  pub fn new<T>(data: T) -> Self {
    let data = Arc::into_raw(Arc::new(data));

    Self {
      _inner: data as *const c_void,
      _unuse: _uunuse_arc::<T>,
      _use: _use_arc::<T>
    }
  }

  pub fn from_raw(arc: *const Self) -> Self {
    let rf = unsafe { &*arc };

    (rf._use)(rf._inner);
    
    Self {
      _use: rf._use,
      _inner: rf._inner,
      _unuse: rf._unuse
    }
  }

  pub fn as_raw(&self) -> *const Self {
    self as _
  }
}

impl Drop for Arced {
  fn drop(&mut self) {
    (self._unuse)(self._inner);
  }
}

// #[repr(C)]
// pub(crate) struct Counter {
//   _mutex: *mut RawMutex,
//   _count: usize
// }

// impl Counter {
//   pub fn new() -> *mut Self {
//     unsafe {
//       let data = libc::aligned_malloc(size_of::<Self>(), align_of::<Self>()) as _;

//       ptr::write(data, Self {
//         _count: 0,
//         _mutex: RawMutex::new()
//       });

//       data
//     }
//   }

//   pub fn free(cnt: *mut Self) {
//     unsafe  {
//       (*(*cnt)._mutex).close();
//       libc::aligned_free((*cnt)._mutex as _);

//       libc::aligned_free(cnt as _);
//     }
//   }
// }

// #[repr(C)]
// pub struct Arcable {
//   data: *mut c_void,
//   bad: bool,
//   counter: *mut Counter,
//   _drop: unsafe extern "C" fn(data: *mut c_void)
// }

// unsafe impl Send for Arcable {}
// unsafe impl Sync for Arcable {}

// unsafe extern "C" fn mfree<T>(data: *mut c_void) {
//   unsafe {
//     ptr::drop_in_place(data as *mut T);

//     libc::aligned_free(data);
//   }
// }

// impl Arcable {
//   /// Creates a arcable struct that should be used with
//   /// Arced to provide a safer counter implementation
//   pub fn new<T: FFISafe>(data: T) -> *mut Self {
//     unsafe {
//       let data_ptr = libc::aligned_malloc(size_of::<T>(), align_of::<T>()) as _;

//       ptr::write(data_ptr, data);

//       let value = Self {
//         data: data_ptr as _,
//         counter: Counter::new(),
//         bad: false,
//         _drop: mfree::<T>
//       };

//       let out_ptr = libc::aligned_malloc(size_of::<Self>(), align_of::<Self>()) as _;

//       ptr::write(out_ptr, value);

//       out_ptr
//     }
//   }

//   pub(crate) unsafe fn use_arc(arc: *mut Arcable) {
//     unsafe {
//       let arc = &*arc;

//       let mutex = &(*(*arc.counter)._mutex);

//       mutex.lock();

//       if arc.bad {
//         panic!("Bad Arc in process of deletion");
//       }

//       (*arc.counter)._count += 1;

//       mutex.unlock();
//     }
//   }

//   pub(crate) unsafe fn unuse_arc(arc_ptr: *mut Arcable) {
//     unsafe {
//       let arc = &mut *arc_ptr;

//       let mutex = &(*(*arc.counter)._mutex);

//       mutex.lock();

//       (*arc.counter)._count -= 1;

//       // Time to delete the arc container
//       if (*arc.counter)._count == 0 {
//         arc.bad = true;
//         (arc._drop)(arc.data);

//         mutex.unlock();
//         Counter::free(arc.counter);

//         libc::aligned_free(arc_ptr as _);
//       } else {
//         mutex.unlock();
//       }
//     }
//   }
// }

// pub struct Arced<T: FFISafe> {
//   data: *mut Arcable,
//   _pha: PhantomData<T>,
// }

// unsafe impl<T: FFISafe> Send for Arced<T> {}
// unsafe impl<T: FFISafe> Sync for Arced<T> {}

// impl<T: FFISafe> Arced<T> {
//   pub unsafe fn new(data: *mut Arcable) -> Self {
//     unsafe { Arcable::use_arc(data) };

//     Self {
//       data,
//       _pha: PhantomData,
//     }
//   }

//   /// DOES NOT DECREMENT THE COUNTER
//   pub fn give_raw(&self) -> *mut Arcable {
//     self.data
//   }
// }

// impl<T: FFISafe> Clone for Arced<T> {
//   fn clone(&self) -> Self {
//     unsafe { Self::new(self.data) }
//   }
// }

// impl<T: FFISafe> Deref for Arced<T> {
//   type Target = T;

//   fn deref(&self) -> &Self::Target {
//     unsafe {
//       &*((*self.data).data as *mut T)
//     }
//   }
// }

// impl<T: FFISafe> Drop for Arced<T> {
//   fn drop(&mut self) {
//     unsafe {
//       Arcable::unuse_arc(self.data);
//     }
//   }
// }