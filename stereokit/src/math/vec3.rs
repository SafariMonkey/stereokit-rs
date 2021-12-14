use derive_more::Deref;
use ffi_helpers::Transmutable;

#[derive(Deref)]
#[repr(transparent)]
pub struct Vec3(pub nalgebra::Vector3<f32>);

unsafe impl Transmutable<stereokit_sys::vec3> for Vec3 {}
