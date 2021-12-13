mod app;
use app::App;

fn main() -> anyhow::Result<()> {
    let mut sk = stereokit::StereoKit::init(App::settings())?;
    let mut app = App::init(sk.state_mut());

    sk.run(&mut app, App::update, App::shutdown);

    Ok(())
}
