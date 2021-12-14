use std::ptr::NonNull;

use super::shader::Shader;

pub struct Material {
    pub(crate) _inst: NonNull<stereokit_sys::_material_t>,
}

impl Material {
    pub fn new(shader: Shader) -> Option<Material> {
        let inst: stereokit_sys::material_t =
            unsafe { stereokit_sys::material_create(shader._inst.as_ptr()) };
        Some(Material {
            _inst: NonNull::new(inst)?,
        })
    }
}
