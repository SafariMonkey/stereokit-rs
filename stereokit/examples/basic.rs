use stereokit::{
    settings::{DepthMode, DisplayBlend, DisplayMode, Log, Settings},
    StereoKit,
};

fn main() -> anyhow::Result<()> {
    let settings = Settings {
        app_name: "stereokit-sys demo".to_owned(),
        assets_folder: "".to_owned(),
        display_preference: DisplayMode::MixedReality,
        blend_preference: DisplayBlend::NONE,
        no_flatscreen_fallback: false,
        disable_flatscreen_mr_sim: false,
        depth_mode: DepthMode::Balanced,
        flatscreen_pos_x: 0,
        flatscreen_pos_y: 0,
        flatscreen_width: 0,
        flatscreen_height: 0,
        log_filter: Log::Diagnostic,
        overlay_app: false,
        overlay_priority: 0,
    };

    let mut i = 0;
    let j = "test".to_owned();

    let mut sk = StereoKit::init(settings, || {
        println!("{} {}", j, i);
        i += 1
    })?;

    sk.run();

    drop(sk);
    println!("after run: {} {}", j, i);

    Ok(())
}
