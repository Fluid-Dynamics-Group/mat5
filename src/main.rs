fn main() {
    //let file = std::path::PathBuf::from("./scripts/octave_output.mat");
    let file = std::path::PathBuf::from("./scripts/matlab_output.mat");
    //let file = std::path::PathBuf::from("./scripts/complex_matlab.mat");
    //let file = std::path::PathBuf::from("./tests/2x2_octave_mirror.mat");

    mat5::parse_file(&file).ok();
}
