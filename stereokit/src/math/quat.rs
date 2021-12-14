use crate::transmute::{TransmuteCopy, TransmuteCopyRev};
use derive_more::Deref;
use ffi_helpers::Transmutable;

use super::vec::Vec3;

#[derive(Deref, Copy, Clone)]
#[repr(transparent)]
pub struct Quat(pub nalgebra::Quaternion<f32>);

unsafe impl Transmutable<stereokit_sys::quat> for Quat {}

impl Quat {
    pub fn look_dir(at: Vec3) -> Self {
        unsafe {
            stereokit_sys::quat_lookat(
                &Vec3::ZERO.transmute_copy_to() as *const _,
                &at.transmute_copy_to() as *const _,
            )
        }
        .transmute_copy_from()
    }
}
