use ffi_helpers::{FieldType, Repr, Transmutable};

use super::{quat::Quat, vec3::Vec3};

#[derive(FieldType, Repr, Transmutable)]
#[transmutable(stereokit_sys::pose_t)]
#[repr(C)]
pub struct Pose {
    pub position: Vec3,
    pub orientation: Quat,
}
