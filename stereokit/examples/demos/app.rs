use stereokit::settings::Settings;
use stereokit::state::StereoKitState;

pub struct App {
    i: i32,
}

impl App {
    pub fn settings() -> Settings {
        Settings {
            app_name: "StereoKit Demos".to_owned(),
            ..Default::default()
        }
    }

    pub fn init(sk: &mut StereoKitState) -> Self {
        Self { i: 32 }
    }

    pub fn update(&mut self, sk: &mut StereoKitState) {
        println!("running: {}", self.i);
        self.i += 1;
    }

    pub fn shutdown(&mut self, sk: &mut StereoKitState) {
        println!("shutting down: {}", self.i);
    }
}
