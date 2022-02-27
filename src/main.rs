use ndarray::Array1;
use ndarray::Array2;

#[derive(mat5::MatFile)]
struct Foo {
    a: Array2<u64>,
    b: Array1<f32>,
}

fn main() {
    //let file = std::path::PathBuf::from("./scripts/octave_output.mat");
    let file = std::path::PathBuf::from("./scripts/matlab_output.mat");
    //let file = std::path::PathBuf::from("./scripts/complex_matlab.mat");
    //let file = std::path::PathBuf::from("./tests/2x2_octave_mirror.mat");

    mat5::parse_file(&file).ok();
}
