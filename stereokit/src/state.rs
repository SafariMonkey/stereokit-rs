use std::ffi::c_void;
// Thanks to https://adventures.michaelfbryan.com/posts/pragmatic-global-state/
use std::marker::PhantomData;

use std::any::Any;
use std::os::raw::c_int;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use once_cell::sync::Lazy;
use snafu::Snafu;

use crate::settings::Settings;

static LIBRARY_IN_USE: AtomicBool = AtomicBool::new(false);

static CAUGHT_PANIC: Lazy<Mutex<Option<Box<dyn Any + Send + 'static>>>> =
    Lazy::new(|| Mutex::new(None));

pub struct StereoKit {
    _not_send: PhantomData<*const ()>,
    needs_shutdown: bool,
}

extern "C" fn callback_trampoline<ST>(payload_ptr: *mut c_void) {
    let payload: &mut (&mut dyn FnMut(&mut ST), &mut ST) =
        unsafe { std::mem::transmute(payload_ptr) };
    let (closure, state) = payload;

    // SAFETY:
    // the call to `resume_unwind` in StereoKit::run means
    // closure variables and state cannot be observed
    // after the panic without catching the panic,
    // which will in turn require them to be UnwindSafe
    let mut closure = AssertUnwindSafe(closure);
    let mut state = AssertUnwindSafe(state);

    let result = std::panic::catch_unwind(move || closure(*state));
    if let Err(panic_payload) = result {
        match CAUGHT_PANIC.lock() {
            Ok(mut inner) => {
                inner.replace(panic_payload);
            }
            Err(_) => {
                // CAUGHT_PANIC is poisoned, so StereoKit::run will panic on read anyway.
            }
        };
        unsafe { stereokit_sys::sk_quit() };
    }
}

impl StereoKit {
    pub fn init(settings: Settings) -> Result<StereoKit, Error> {
        if LIBRARY_IN_USE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            == Ok(false)
        {
            let init_success = unsafe { stereokit_sys::sk_init(settings.as_native()) };
            if init_success != 1 {
                return Err(Error::FailedInititalization);
            };
            Ok(Self {
                _not_send: PhantomData,
                needs_shutdown: true,
            })
        } else {
            Err(Error::AlreadyInUse)
        }
    }

    pub fn run<ST, U, S>(mut self, state: &mut ST, mut update: U, mut shutdown: S)
    where
        U: FnMut(&mut ST),
        S: FnMut(&mut ST),
    {
        let mut update_ref: (&mut dyn FnMut(&mut ST), &mut ST) = (&mut update, state);
        let update_raw = &mut update_ref as *mut (&mut dyn FnMut(&mut ST), &mut ST) as *mut c_void;
        let mut shutdown_ref: (&mut dyn FnMut(&mut ST), &mut ST) = (&mut shutdown, state);
        let shutdown_raw =
            &mut shutdown_ref as *mut (&mut dyn FnMut(&mut ST), &mut ST) as *mut c_void;

        self.needs_shutdown = false;
        unsafe {
            stereokit_sys::sk_run_data(
                Some(callback_trampoline::<ST>),
                update_raw,
                Some(callback_trampoline::<ST>),
                shutdown_raw,
            );
        }

        let caught_panic = CAUGHT_PANIC.lock().unwrap().take();
        if let Some(panic_payload) = caught_panic {
            std::panic::resume_unwind(panic_payload);
        }
    }
}

impl Drop for StereoKit {
    fn drop(&mut self) {
        if self.needs_shutdown {
            unsafe { stereokit_sys::sk_shutdown() };
        }
        LIBRARY_IN_USE.store(false, Ordering::SeqCst);
    }
}

/// The various error cases that may be encountered while using this library.
#[derive(Debug, Copy, Clone, PartialEq, Snafu)]
pub enum Error {
    #[snafu(display("StereoKit is already in use"))]
    AlreadyInUse,
    #[snafu(display("StereoKit inititalization failed"))]
    FailedInititalization,
}

#[cfg(test)]
mod tests {
    use super::*;

    // if this assertion fails, the error looks like:
    // consider giving this pattern the explicit type `fn()` ...
    static_assertions::assert_not_impl_any!(StereoKit: Send, Sync);
}
