use derive_more::{Deref, Neg};
use ffi_helpers::Transmutable;
use nalgebra::{Vector2, Vector3, Vector4};

#[derive(Deref, Copy, Clone, Neg)]
#[repr(transparent)]
pub struct Vec2(pub nalgebra::Vector2<f32>);

unsafe impl Transmutable<stereokit_sys::vec2> for Vec2 {}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self(Vector2::new(x, y))
    }
    pub const ZERO: Vec2 = Vec2::new(0., 0.);
    pub const ONE: Vec2 = Vec2::new(1., 1.);
    pub const UNIT_X: Vec2 = Vec2::new(1., 0.);
    pub const UNIT_Y: Vec2 = Vec2::new(0., 1.);
}

#[derive(Deref, Copy, Clone, Neg)]
#[repr(transparent)]
pub struct Vec3(pub nalgebra::Vector3<f32>);

unsafe impl Transmutable<stereokit_sys::vec3> for Vec3 {}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vector3::new(x, y, z))
    }

    pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);
    pub const ONE: Vec3 = Vec3::new(1., 1., 1.);
    pub const UP: Vec3 = Vec3::new(0., 1., 0.);
    pub const FORWARD: Vec3 = Vec3::new(0., 0., -1.);
    pub const RIGHT: Vec3 = Vec3::new(1., 0., 0.);
    pub const UNIT_X: Vec3 = Vec3::new(1., 0., 0.);
    pub const UNIT_Y: Vec3 = Vec3::new(0., 1., 0.);
    pub const UNIT_Z: Vec3 = Vec3::new(0., 0., 1.);
}

#[derive(Deref, Copy, Clone, Neg)]
#[repr(transparent)]
pub struct Vec4(pub nalgebra::Vector4<f32>);

unsafe impl Transmutable<stereokit_sys::vec4> for Vec4 {}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vector4::new(x, y, z, w))
    }

    pub const UNIT_X: Vec4 = Vec4::new(1., 0., 0., 0.);
    pub const UNIT_Y: Vec4 = Vec4::new(0., 1., 0., 0.);
    pub const UNIT_Z: Vec4 = Vec4::new(0., 0., 1., 0.);
    pub const UNIT_W: Vec4 = Vec4::new(0., 0., 0., 1.);
}
