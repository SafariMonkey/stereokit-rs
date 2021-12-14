use stereokit::assets::material::Material;
use stereokit::assets::mesh::Mesh;
use stereokit::assets::model::Model;
use stereokit::assets::shader::Shader;
use stereokit::math::matrix::Matrix;
use stereokit::math::pose::Pose;
use stereokit::math::quat::Quat;
use stereokit::math::vec::{Vec2, Vec3};
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
        let floor_mat =
            Material::new(Shader::from_file("floor_shader.hlsl").expect("failed to load shader"))
                .expect("failed to make material");
        let floor_tr = Matrix::trs(Vec3::new(0.0, -1.5, 0.0), Quat::IDENTITY, Vec3::ONE);
        let floor_mesh = Model::from_mesh(
            Mesh::generate_plane(Vec2::new(40., 40.), Vec3::UP, Vec3::FORWARD, 0)
                .expect("failed to generate mesh"),
            floor_mat,
        )
        .expect("failed to make floor mesh model");
        Self {
            floor_mesh,
            floor_tr,
            demo_select_pose,
        }
    }

    pub fn update(&mut self, sk: &mut StereoKitState) {
        // println!("running: {}", self.i);
        // self.i += 1;
    }

    pub fn shutdown(&mut self, sk: &mut StereoKitState) {
        println!("shutting down");
    }
}

trait Demo {
    fn initialize();
    fn update();
    fn shutdown();
}
