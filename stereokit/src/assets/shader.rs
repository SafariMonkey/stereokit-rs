use std::{ffi::CString, ptr::NonNull};

pub struct Shader {
    pub(crate) _inst: NonNull<stereokit_sys::_shader_t>,
}

impl Shader {
    pub fn from_file(filename: &str) -> Option<Shader> {
        let ffi_filename = CString::new(filename).expect("null byte in filename");
        let inst: stereokit_sys::shader_t =
            unsafe { stereokit_sys::shader_create_file(ffi_filename.as_ptr()) };
        Some(Shader {
            _inst: NonNull::new(inst)?,
        })
    }
}
