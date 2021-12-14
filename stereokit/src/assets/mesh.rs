use std::ptr::NonNull;

use crate::{
    math::vec::{Vec2, Vec3},
    transmute::Transmute,
};

pub struct Mesh {
    pub(crate) _inst: NonNull<stereokit_sys::_mesh_t>,
}

impl Mesh {
    pub fn new() -> Option<Mesh> {
        let inst: stereokit_sys::mesh_t = unsafe { stereokit_sys::mesh_create() };
        Self::_from_inst(inst)
    }
    fn _from_inst(inst: stereokit_sys::mesh_t) -> Option<Mesh> {
        Some(Mesh {
            _inst: NonNull::new(inst)?,
        })
    }

    pub fn generate_plane(
        dimensions: Vec2,
        plane_normal: Vec3,
        plane_top_direction: Vec3,
        subdivisions: i32,
    ) -> Option<Mesh> {
        Mesh::_from_inst(unsafe {
            stereokit_sys::mesh_gen_plane(
                dimensions.transmute_copy_to(),
                plane_normal.transmute_copy_to(),
                plane_top_direction.transmute_copy_to(),
                subdivisions,
            )
        })
    }
}
