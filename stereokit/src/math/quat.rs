use ffi_helpers::{FieldType, Repr, Transmutable};

#[derive(FieldType, Repr, Transmutable)]
#[transmutable(stereokit_sys::quat)]
#[repr(C)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
