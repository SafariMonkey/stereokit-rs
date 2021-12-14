use crate::transmute::{Transmute, TransmuteRev};
use derive_more::Deref;
use ffi_helpers::Transmutable;

use super::vec::Vec3;

#[derive(Deref, Copy, Clone)]
#[repr(transparent)]
pub struct Quat(pub nalgebra::Quaternion<f32>);

unsafe impl Transmutable<stereokit_sys::quat> for Quat {}

impl Quat {
    pub const IDENTITY: Quat = Quat(nalgebra::Quaternion::new(1., 0., 0., 0.));

    pub fn look_dir(at: Vec3) -> Self {
        Self::look_at(Vec3::ZERO, at)
    }

    pub fn look_at(from: Vec3, at: Vec3) -> Self {
        let from = from.transmute_copy_to();
        let at = at.transmute_copy_to();
        unsafe { stereokit_sys::quat_lookat(&from as *const _, &at as *const _) }
            .transmute_copy_from()
    }
}
