use stereokit::{settings::Settings, StereoKit};

fn main() -> anyhow::Result<()> {
    let settings = Settings {
        app_name: "stereokit demo".to_owned(),
        ..Default::default()
    };

    let sk = StereoKit::init(settings)?;

    let mut i = 0;
    let j = "test".to_owned();

    sk.run(
        &mut (&mut i, &j),
        |(i, j)| {
            println!("{} {}", j, i);
            if **i >= 32 {
                // panics trigger an application quit:
                // panic!("{} >= 32", i)
            }
            **i += 1
        },
        |(i, j)| println!("shutting down: {} {}", j, i),
    );

    println!("after run: {} {}", j, i);

    Ok(())
}
