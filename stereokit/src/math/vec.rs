use derive_more::Deref;
use ffi_helpers::Transmutable;

#[derive(Deref)]
#[repr(transparent)]
pub struct Vec2(pub nalgebra::Vector2<f32>);

unsafe impl Transmutable<stereokit_sys::vec2> for Vec2 {}

#[derive(Deref)]
#[repr(transparent)]
pub struct Vec3(pub nalgebra::Vector3<f32>);

unsafe impl Transmutable<stereokit_sys::vec3> for Vec3 {}

#[derive(Deref)]
#[repr(transparent)]
pub struct Vec4(pub nalgebra::Vector4<f32>);

unsafe impl Transmutable<stereokit_sys::vec4> for Vec4 {}
