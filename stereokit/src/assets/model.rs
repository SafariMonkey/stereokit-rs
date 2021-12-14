use std::ptr::NonNull;

use super::{material::Material, mesh::Mesh};

pub struct Model {
    pub(crate) _inst: NonNull<stereokit_sys::_model_t>,
}

impl Model {
    pub fn from_mesh(mesh: Mesh, material: Material) -> Option<Model> {
        let inst: stereokit_sys::model_t = unsafe {
            stereokit_sys::model_create_mesh(mesh._inst.as_ptr(), material._inst.as_ptr())
        };
        Some(Model {
            _inst: NonNull::new(inst)?,
        })
    }
}
