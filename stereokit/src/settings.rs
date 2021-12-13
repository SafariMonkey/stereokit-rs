use derive_more::{BitAnd, BitOr};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Settings {
    pub app_name: String,
    pub assets_folder: String,
    pub display_preference: DisplayMode,
    pub blend_preference: DisplayBlend,
    pub no_flatscreen_fallback: bool,
    pub depth_mode: DepthMode,
    pub log_filter: LogLevel,
    pub overlay_app: bool,
    pub overlay_priority: u32,
    pub flatscreen_pos_x: i32,
    pub flatscreen_pos_y: i32,
    pub flatscreen_width: i32,
    pub flatscreen_height: i32,
    pub disable_flatscreen_mr_sim: bool,
    pub disable_unfocused_sleep: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            app_name: "StereoKit App".to_owned(),
            assets_folder: "Assets".to_owned(),
            display_preference: DisplayMode::MixedReality,
            blend_preference: DisplayBlend::NONE,
            no_flatscreen_fallback: false,
            disable_flatscreen_mr_sim: false,
            disable_unfocused_sleep: false,
            depth_mode: DepthMode::Balanced,
            flatscreen_pos_x: 0,
            flatscreen_pos_y: 0,
            flatscreen_width: 0,
            flatscreen_height: 0,
            log_filter: LogLevel::Diagnostic,
            overlay_app: false,
            overlay_priority: 0,
        }
    }
}

impl Settings {
    pub(crate) fn as_native(&self) -> stereokit_sys::sk_settings_t {
        stereokit_sys::sk_settings_t {
            app_name: ustr::ustr(&self.app_name).as_char_ptr(),
            assets_folder: ustr::ustr(&self.assets_folder).as_char_ptr(),
            display_preference: self.display_preference as u32,
            blend_preference: self.blend_preference.0,
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
pub enum LogLevel {
    None = stereokit_sys::log__log_none,
    Diagnostic = stereokit_sys::log__log_diagnostic,
    Inform = stereokit_sys::log__log_inform,
    Warning = stereokit_sys::log__log_warning,
    Error = stereokit_sys::log__log_error,
}

#[derive(Debug, Copy, Clone, BitAnd, BitOr)]
pub struct Display(u32);
impl Display {
    pub const NONE: Self = Self(stereokit_sys::display__display_none);
    pub const OPAQUE: Self = Self(stereokit_sys::display__display_opaque);
    pub const ADDITIVE: Self = Self(stereokit_sys::display__display_additive);
    pub const BLEND: Self = Self(stereokit_sys::display__display_blend);
    pub const PASSTHROUGH: Self = Self(stereokit_sys::display__display_passthrough);

    pub const ANY_TRANSPARENT: Self = Self(stereokit_sys::display__display_any_transparent);
}

#[derive(Debug, Copy, Clone, BitAnd, BitOr)]
pub struct DisplayBlend(u32);
impl DisplayBlend {
    pub const NONE: Self = Self(stereokit_sys::display_blend__display_blend_none);
    pub const OPAQUE: Self = Self(stereokit_sys::display_blend__display_blend_opaque);
    pub const ADDITIVE: Self = Self(stereokit_sys::display_blend__display_blend_additive);
    pub const BLEND: Self = Self(stereokit_sys::display_blend__display_blend_blend);

    pub const ANY_TRANSPARENT: Self =
        Self(stereokit_sys::display_blend__display_blend_any_transparent);
}

#[derive(Debug, Copy, Clone, BitAnd, BitOr)]
pub struct RenderLayer(u32);
impl RenderLayer {
    pub const L0: Self = Self(stereokit_sys::render_layer__render_layer_0);
    pub const L1: Self = Self(stereokit_sys::render_layer__render_layer_1);
    pub const L2: Self = Self(stereokit_sys::render_layer__render_layer_2);
    pub const L3: Self = Self(stereokit_sys::render_layer__render_layer_3);
    pub const L4: Self = Self(stereokit_sys::render_layer__render_layer_4);
    pub const L5: Self = Self(stereokit_sys::render_layer__render_layer_5);
    pub const L6: Self = Self(stereokit_sys::render_layer__render_layer_6);
    pub const L7: Self = Self(stereokit_sys::render_layer__render_layer_7);
    pub const L8: Self = Self(stereokit_sys::render_layer__render_layer_8);
    pub const L9: Self = Self(stereokit_sys::render_layer__render_layer_9);
    pub const VFX: Self = Self(stereokit_sys::render_layer__render_layer_vfx);

    pub const ALL: Self = Self(stereokit_sys::render_layer__render_layer_all);
    pub const ALL_REGULAR: Self = Self(stereokit_sys::render_layer__render_layer_all_regular);
}
