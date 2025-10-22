use std::sync::{Arc, Mutex};

use cursive::view::{View, ViewWrapper};
use cursive::{CbSink, Cursive, Printer, wrap_impl};

use crate::structs::{Authentication, Config};
use crate::ui::Ptr;

pub struct OnAuthStateUpdate<V: View, F: Fn(&mut Cursive) + 'static> {
  inner: V,
  sink: CbSink,
  last_state: Arc<Mutex<Authentication>>,
  callback: F,
}

impl<V: View, F: Fn(&mut Cursive) + 'static> OnAuthStateUpdate<V, F> {
  pub fn new(inner: V, siv: &mut Cursive, callback: F) -> Self {
    Self {
      inner,
      sink: siv.cb_sink().clone(),
      last_state: Arc::new(Mutex::new({
        let data: &mut Ptr<Config> = siv.user_data().unwrap();

        match data.authentication {
          Authentication::Account { .. } => Authentication::TokenBased,
          Authentication::OpenToAll => Authentication::TokenBased,
          Authentication::TokenBased => Authentication::OpenToAll,
        }
      })),
      callback,
    }
  }
}

impl<V: View, T: Fn(&mut Cursive) + Send + Sync + 'static> ViewWrapper for OnAuthStateUpdate<V, T> {
  wrap_impl!(self.inner: V);

  fn wrap_draw(&self, printer: &Printer) {
    let cb_ref = &self.callback;

    let cb_ref: &'static T = unsafe { &*(cb_ref as *const T) };

    let state = self.last_state.clone();

    _ = self.sink.clone().send(Box::new(move |x| {
      let mut lock = state.lock().unwrap();

      let data: &mut Ptr<Config> = x.user_data().unwrap();
      let auth = &data.authentication;

      if &*lock != auth {
        *lock = auth.clone();

        (cb_ref)(x);
      }
    }));

    self.inner.draw(printer);
  }
}
