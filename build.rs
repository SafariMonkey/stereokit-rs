// adapted from https://github.com/kornelski/mozjpeg-sys/blob/efc8d99dcebc4d725608f782dc4fdaf7361d2b68/src/build.rs

use std::env;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

fn compiler(config_dir: &Path, vendor: &Path) -> cc::Build {
    let mut c = cc::Build::new();
    c.include(&config_dir);
    c.include(&vendor);
    c.pic(true);
    c.cpp(true);
    c.warnings(false);

    if let Ok(target_cpu) = env::var("TARGET_CPU") {
        c.flag_if_supported(&format!("-march={}", target_cpu));
    }

    if cfg!(feature = "unwinding") {
        c.flag_if_supported("-fexceptions");
    }

    c
}

fn main() {
    let root =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let root = dunce::canonicalize(root).expect("root dir");
    let vendor = root.join("vendor");
    let config_dir = vendor.join("include");

    // cc crate needs emscripten target to use correct `ar`
    if env::var("TARGET").map_or(false, |t| t == "wasm32-unknown-unknown") {
        println!(
            "cargo:warning=If the build fails, try using wasm32-unknown-emscripten target instead"
        );
        eprintln!("If the build fails, try using wasm32-unknown-emscripten target instead");
    }

    println!("cargo:include={}", config_dir.to_str().expect("inc"));
    let mut c = compiler(&config_dir, &vendor);

    c.include(
        env::var_os("DEP_REACTPHYSICS3D_INCLUDE")
            .expect("DEP_REACTPHYSICS3D_INCLUDE should have been set by reactphysics3d-sys"),
    );

    let source_dirs = [
        "vendor/StereoKitC",
        "vendor/StereoKitC/libraries",
        "vendor/StereoKitC/tools",
        "vendor/StereoKitC/systems",
        "vendor/StereoKitC/systems/hand",
        "vendor/StereoKitC/systems/platform",
        "vendor/StereoKitC/asset_types",
    ];
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "linux" {
        c.include("vendor/StereoKitC/lib/include_no_win");
    }
    for dir in source_dirs.iter() {
        let dir = Path::new(dir);
        assert!(
            dir.is_dir(),
            "source directory is missing. Maybe you need to run `git submodule update --init`?"
        );

        for entry in dir
            .read_dir()
            .unwrap()
            .flat_map(Result::ok)
            .filter(|path| matches!(path.path().extension(), Some( ext) if ext == "cpp"))
        {
            c.file(entry.path());
        }
    }

    c.compile("stereokit");
}
