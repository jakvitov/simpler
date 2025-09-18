use std::path::PathBuf;

///Used for path setup to testing mps
pub enum CorrectMps {
        CORRECT,
        INCORRECT
}

pub fn setup_path_to_mps(mps_name: &str, correct_mps: CorrectMps) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/data/mps");
        match correct_mps {
                CorrectMps::CORRECT => path.push("correct"),
                CorrectMps::INCORRECT => path.push("incorrect"),
        }
        path.push(mps_name);
        path
}