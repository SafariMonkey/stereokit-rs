use bitflags::bitflags;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Settings {
    pub app_name: String,
    pub assets_folder: String,
    pub display_preference: DisplayMode,
    pub blend_preference: DisplayBlend,
    pub no_flatscreen_fallback: bool,
    pub depth_mode: DepthMode,
    pub log_filter: Log,
    pub overlay_app: bool,
    pub overlay_priority: u32,
    pub flatscreen_pos_x: i32,
    pub flatscreen_pos_y: i32,
    pub flatscreen_width: i32,
    pub flatscreen_height: i32,
    pub disable_flatscreen_mr_sim: bool,
    pub disable_unfocused_sleep: bool,
}

impl Settings {
    pub(crate) fn as_native(&self) -> stereokit_sys::sk_settings_t {
        stereokit_sys::sk_settings_t {
            app_name: ustr::ustr(&self.app_name).as_char_ptr(),
            assets_folder: ustr::ustr(&self.assets_folder).as_char_ptr(),
            display_preference: self.display_preference as u32,
            blend_preference: self.blend_preference.bits,
            no_flatscreen_fallback: self.no_flatscreen_fallback as i32,
            depth_mode: self.depth_mode as u32,
            log_filter: self.log_filter as u32,
            overlay_app: self.overlay_app as i32,
            overlay_priority: self.overlay_priority,
            flatscreen_pos_x: self.flatscreen_pos_x,
            flatscreen_pos_y: self.flatscreen_pos_y,
            flatscreen_width: self.flatscreen_width,
            flatscreen_height: self.flatscreen_height,
            disable_flatscreen_mr_sim: self.disable_flatscreen_mr_sim as i32,
            disable_unfocused_sleep: self.disable_unfocused_sleep as i32,
            android_java_vm: std::ptr::null_mut(),
            android_activity: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum DisplayMode {
    MixedReality = stereokit_sys::display_mode__display_mode_mixedreality,
    Flatscreen = stereokit_sys::display_mode__display_mode_flatscreen,
    None = stereokit_sys::display_mode__display_mode_none,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum DepthMode {
    Balanced = stereokit_sys::depth_mode__depth_mode_balanced,
    D16 = stereokit_sys::depth_mode__depth_mode_d16,
    D32 = stereokit_sys::depth_mode__depth_mode_d32,
    Stencil = stereokit_sys::depth_mode__depth_mode_stencil,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Log {
    None = stereokit_sys::log__log_none,
    Diagnostic = stereokit_sys::log__log_diagnostic,
    Inform = stereokit_sys::log__log_inform,
    Warning = stereokit_sys::log__log_warning,
    Error = stereokit_sys::log__log_error,
}

bitflags! {
    pub struct Display: u32 {
        const NONE = stereokit_sys::display__display_none;
        const OPAQUE = stereokit_sys::display__display_opaque;
        const ADDITIVE = stereokit_sys::display__display_additive;
        const BLEND = stereokit_sys::display__display_blend;
        const PASSTHROUGH = stereokit_sys::display__display_passthrough;

        const ANY_TRANSPARENT = stereokit_sys::display__display_any_transparent;
    }

    pub struct DisplayBlend: u32 {
        const NONE = stereokit_sys::display_blend__display_blend_none;
        const OPAQUE = stereokit_sys::display_blend__display_blend_opaque;
        const ADDITIVE = stereokit_sys::display_blend__display_blend_additive;
        const BLEND = stereokit_sys::display_blend__display_blend_blend;

        const ANY_TRANSPARENT = stereokit_sys::display_blend__display_blend_any_transparent;
    }

    pub struct RenderLayer: u32 {
        const L0 = stereokit_sys::render_layer__render_layer_0;
        const L1 = stereokit_sys::render_layer__render_layer_1;
        const L2 = stereokit_sys::render_layer__render_layer_2;
        const L3 = stereokit_sys::render_layer__render_layer_3;
        const L4 = stereokit_sys::render_layer__render_layer_4;
        const L5 = stereokit_sys::render_layer__render_layer_5;
        const L6 = stereokit_sys::render_layer__render_layer_6;
        const L7 = stereokit_sys::render_layer__render_layer_7;
        const L8 = stereokit_sys::render_layer__render_layer_8;
        const L9 = stereokit_sys::render_layer__render_layer_9;
        const VFX = stereokit_sys::render_layer__render_layer_vfx;

        const ALL = stereokit_sys::render_layer__render_layer_all;
        const ALL_REGULAR = stereokit_sys::render_layer__render_layer_all_regular;
    }
}
