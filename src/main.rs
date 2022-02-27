#[derive(mat5::MatFile)]
struct PaddingName {
    inner: ndarray::Array2<u32>,
    inner2: ndarray::Array3<u32>,
    inner3: ndarray::Array4<u32>,
    inner4: ndarray::Array1<u32>,
}

fn main() {
    //let file = std::path::PathBuf::from("./scripts/octave_output.mat");
    let file = std::path::PathBuf::from("./scripts/matlab_output.mat");
    //let file = std::path::PathBuf::from("./scripts/complex_matlab.mat");
    //let file = std::path::PathBuf::from("./tests/2x2_octave_mirror.mat");

    mat5::parse_file(&file).ok();
}
