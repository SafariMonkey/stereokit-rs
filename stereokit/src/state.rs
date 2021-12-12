use std::ffi::c_void;
// Thanks to https://adventures.michaelfbryan.com/posts/pragmatic-global-state/
use std::marker::PhantomData;

use std::any::Any;

use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, Ordering};

use snafu::Snafu;

use crate::settings::Settings;

static LIBRARY_IN_USE: AtomicBool = AtomicBool::new(false);

type PanicPayload = Box<dyn Any + Send + 'static>;

pub struct StereoKit {
    _not_send: PhantomData<*const ()>,
    needs_shutdown: bool,
}

/// SAFETY: payload_ptr must point to a value of type
/// `(&mut F, &mut ST, &mut Option<PanicPayload>)`.
/// It must also not be called synchronously with itself
/// or any other callback using the same parameters (due to &mut).
/// If `caught_panic` is written to, `F` and `ST` are
/// panic-poisoned, and the panic should likely be propagated.
unsafe extern "C" fn callback_trampoline<F, ST>(payload_ptr: *mut c_void)
where
    F: FnMut(&mut ST),
{
    let payload = &mut *(payload_ptr as *mut (&mut F, &mut ST, &mut Option<PanicPayload>));
    let (closure, state, caught_panic) = payload;

    if caught_panic.is_some() {
        // we should consider the state poisoned and not run the callback
        return;
    }

    // SAFETY:
    // the caller must ensure closure variables and state cannot be observed
    // after the panic without catching the panic,
    // which will in turn require them to be UnwindSafe
    let mut closure = AssertUnwindSafe(closure);
    let mut state = AssertUnwindSafe(state);

    let result = std::panic::catch_unwind(move || closure(*state));
    if let Err(panic_payload) = result {
        caught_panic.replace(panic_payload);
        stereokit_sys::sk_quit();
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
        // use one variable so shutdown doesn't run if update panics
        let mut caught_panic = Option::<PanicPayload>::None;

        let mut update_ref: (&mut U, &mut ST, &mut Option<PanicPayload>) =
            (&mut update, state, &mut caught_panic);
        let update_raw =
            &mut update_ref as *mut (&mut U, &mut ST, &mut Option<PanicPayload>) as *mut c_void;

        let mut shutdown_ref: (&mut S, &mut ST, &mut Option<PanicPayload>) =
            (&mut shutdown, state, &mut caught_panic);
        let shutdown_raw =
            &mut shutdown_ref as *mut (&mut S, &mut ST, &mut Option<PanicPayload>) as *mut c_void;

        self.needs_shutdown = false;
        unsafe {
            stereokit_sys::sk_run_data(
                Some(callback_trampoline::<U, ST>),
                update_raw,
                Some(callback_trampoline::<S, ST>),
                shutdown_raw,
            );
        }

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
