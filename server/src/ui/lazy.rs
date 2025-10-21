use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use cursive::view::{View, ViewWrapper};
use cursive::{CbSink, Cursive, Printer, wrap_impl};

pub struct OnVisible<V: View, F: Fn(&mut Cursive) + 'static> {
  inner: V,
  sink: CbSink,
  last_call: Arc<Mutex<u64>>,
  callback: F,
}

impl<V: View, F: Fn(&mut Cursive) + 'static> OnVisible<V, F> {
  pub fn new(inner: V, siv: &Cursive, callback: F) -> Self {
    Self {
      inner,
      sink: siv.cb_sink().clone(),
      last_call: Arc::new(Mutex::new(
        0,
      )),
      callback: callback,
    }
  }
}

impl<V: View, T: Fn(&mut Cursive) + Send + Sync + 'static> ViewWrapper for OnVisible<V, T> {
  wrap_impl!(self.inner: V);

  fn wrap_draw(&self, printer: &Printer) {
    let cb_ref = &self.callback;

    let cb_ref: &'static T = unsafe { &*(cb_ref as *const T) };

    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();

    let mut lock = self.last_call.lock().unwrap();

    if (*lock + 1) < now {
      *lock = now;
      _ = self.sink.clone().send(Box::new(|x| {
        (cb_ref)(x);
      }));
    }

    self.inner.draw(printer);
  }
}
