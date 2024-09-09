use std::{env, fs, path::{Path, PathBuf}};

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn main() {
    let target_dir = get_output_path();
    // should probably automate this
    let src = Path::join(&env::current_dir().unwrap(), "resources").join("steam_logo.png");
    let dest = Path::join(Path::new(&target_dir), Path::new("steam_logo.png"));
    fs::copy(src, dest).unwrap();
}
