use std::path::PathBuf;

pub fn setup_path_to_mps(mps_name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mps");
        path.push(mps_name);
        path
}