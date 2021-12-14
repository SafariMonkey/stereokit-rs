use stereokit::assets::model::Model;
use stereokit::math::matrix::Matrix;
use stereokit::math::pose::Pose;
use stereokit::settings::{DisplayBlend, DisplayMode, LogLevel, Settings};
use stereokit::state::StereoKitState;

pub struct App {
    floor_mesh: Model,
    floor_tr: Matrix,
    demo_select_pose: Pose,
}

impl App {
    pub fn settings() -> Settings {
        Settings {
            app_name: "StereoKit Rust".to_owned(),
            assets_folder: "Assets".to_owned(),
            blend_preference: DisplayBlend::ANY_TRANSPARENT,
            display_preference: DisplayMode::MixedReality,
            log_filter: LogLevel::Diagnostic,
            ..Default::default()
        }
    }

    pub fn init(sk: &mut StereoKitState) -> Self {
        let demo_select_pose = Pose {
            position: Vec3::new(0.0, 0.0, -0.6),
            orientation: Quat::look_dir(-Vec3::FORWARD),
        };
        Self {
            floor_mesh: todo!(),
            floor_tr: todo!(),
            demo_select_pose,
        }
    }

    pub fn update(&mut self, sk: &mut StereoKitState) {
        println!("running: {}", self.i);
        self.i += 1;
    }

    pub fn shutdown(&mut self, sk: &mut StereoKitState) {
        println!("shutting down: {}", self.i);
    }
}

trait Demo {
    fn initialize();
    fn update();
    fn shutdown();
}
