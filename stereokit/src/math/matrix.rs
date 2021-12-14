use std::str::Matches;

use derive_more::Deref;
use ffi_helpers::Transmutable;

use crate::transmute::{Transmute, TransmuteRev};

use super::{quat::Quat, vec::Vec3};

#[derive(Deref, Copy, Clone)]
#[repr(transparent)]
pub struct Matrix(pub nalgebra::Matrix4<f32>);

unsafe impl Transmutable<stereokit_sys::matrix> for Matrix {}

impl Matrix {
    pub fn trs(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        unsafe {
            stereokit_sys::matrix_trs(
                translation.transmute_ref_to() as *const _,
                rotation.transmute_ref_to() as *const _,
                scale.transmute_ref_to() as *const _,
            )
            .transmute_copy_from()
        }
    }
}
