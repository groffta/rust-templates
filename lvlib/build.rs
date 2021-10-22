use cbindgen::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = crate_dir.join("target").join(env::var("PROFILE").unwrap());
    let header_name = format!("{}.h", env::var("CARGO_PKG_NAME").unwrap()).replace("-","_");

    let config = 
    if let Ok(config) = Config::from_file(crate_dir.join("cbindgen.toml")) {
        config
    } else {
        Config {
            language: cbindgen::Language::C,
            parse: cbindgen::ParseConfig {
                parse_deps: false,
                clean: true,
                expand: cbindgen::ParseExpandConfig {
                    all_features: false,
                    default_features: true,
                    features: None,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    };

    cbindgen::generate_with_config(crate_dir, config)
        .unwrap()
        .write_to_file(target_dir.join(header_name));

}