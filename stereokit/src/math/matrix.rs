use derive_more::Deref;
use ffi_helpers::Transmutable;

#[derive(Deref, Copy, Clone)]
#[repr(transparent)]
pub struct Matrix(pub nalgebra::Matrix4<f32>);

unsafe impl Transmutable<stereokit_sys::matrix> for Matrix {}
