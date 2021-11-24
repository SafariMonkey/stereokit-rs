use c_str_macro::c_str;
use std::ptr::null_mut;
use stereokit_sys as sk_sys;

fn main() {
    let settings = stereokit_sys::sk_settings_t {
        app_name: c_str!("stereokit-sys demo").as_ptr(),
        assets_folder: c_str!("").as_ptr(),
        display_preference: sk_sys::display_mode__display_mode_mixedreality,
        blend_preference: sk_sys::display_blend__display_blend_none,
        no_flatscreen_fallback: 0,
        disable_flatscreen_mr_sim: 0,
        depth_mode: sk_sys::depth_mode__depth_mode_balanced,
        flatscreen_pos_x: 0,
        flatscreen_pos_y: 0,
        flatscreen_width: 0,
        flatscreen_height: 0,
        log_filter: sk_sys::log__log_diagnostic,
        android_java_vm: null_mut(),
        android_activity: null_mut(),
        overlay_app: 0,
        overlay_priority: 0,
    };

    unsafe { sk_sys::sk_init(settings) };

    while unsafe { sk_sys::sk_step(Some(step)) } == 1 {}

    unsafe { sk_sys::sk_shutdown() };
}

extern "C" fn step() {}
