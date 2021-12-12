use std::ffi::c_void;
// Thanks to https://adventures.michaelfbryan.com/posts/pragmatic-global-state/
use std::marker::PhantomData;

use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering};

use snafu::Snafu;

use crate::settings::Settings;

static LIBRARY_IN_USE: AtomicBool = AtomicBool::new(false);

pub struct StereoKit<'f> {
    _not_send: PhantomData<*const ()>,
    raw_callback: *mut Box<dyn FnMut() + 'f>,
}

extern "C" fn callback_trampoline(arg: *mut c_void) {
    let closure: &mut Box<dyn FnMut()> = unsafe { std::mem::transmute(arg) };
    closure()
}

impl<'f> StereoKit<'f> {
    pub fn init<F>(settings: Settings, callback: F) -> Result<StereoKit<'f>, Error>
    where
        F: FnMut(),
        F: 'f,
    {
        if LIBRARY_IN_USE.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            == Ok(false)
        {
            unsafe { stereokit_sys::sk_init(settings.as_native()) };

            let boxed_callback: Box<Box<dyn FnMut() + 'f>> = Box::new(Box::new(callback));
            let raw_callback = Box::into_raw(boxed_callback);
            unsafe {
                stereokit_sys::sk_set_app_callback(
                    Some(callback_trampoline),
                    raw_callback as *mut _,
                );
            }

            Ok(Self {
                _not_send: PhantomData,
                raw_callback,
            })
        } else {
            Err(Error::AlreadyInUse)
        }
    }

    pub fn run(&mut self) {
        while unsafe { stereokit_sys::sk_step() } == 1 {}
    }
}

impl<'f> Drop for StereoKit<'f> {
    fn drop(&mut self) {
        unsafe { stereokit_sys::sk_shutdown() };
        // drop the callback
        let _: Box<Box<dyn FnMut()>> = unsafe { Box::from_raw(self.raw_callback as *mut _) };
        LIBRARY_IN_USE.store(false, Ordering::SeqCst);
    }
}

/// The various error cases that may be encountered while using this library.
#[derive(Debug, Copy, Clone, PartialEq, Snafu)]
pub enum Error {
    #[snafu(display("StereoKit is already in use"))]
    AlreadyInUse,
}

#[cfg(test)]
mod tests {
    use super::*;

    // if this assertion fails, the error looks like:
    // consider giving this pattern the explicit type `fn()` ...
    static_assertions::assert_not_impl_any!(StereoKit: Send, Sync);
}
