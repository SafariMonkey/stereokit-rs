use ffi_helpers::{FieldType, Repr, Transmutable};

#[derive(FieldType, Repr, Transmutable)]
#[transmutable(stereokit_sys::vec3)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
