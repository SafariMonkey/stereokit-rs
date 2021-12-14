use derive_more::Deref;
use ffi_helpers::Transmutable;

#[derive(Deref)]
#[repr(transparent)]
pub struct Quat(pub nalgebra::Quaternion<f32>);

unsafe impl Transmutable<stereokit_sys::quat> for Quat {}
