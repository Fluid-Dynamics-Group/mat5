use ndarray::Array1;
use ndarray::Array2;

#[derive(mat5::MatFile)]
//#[mat5(deref(name="something"))]
//#[mat5(deref)]
struct Foo {
    #[mat5(deref)]
    a: Wrap,
    //#[mat5(deref)]
    b: Array1<f32>,
}

struct Wrap(Array2<u64>);

impl std::ops::Deref for Wrap {
    type Target = Array2<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    //let file = std::path::PathBuf::from("./scripts/octave_output.mat");
    let file = std::path::PathBuf::from("./scripts/matlab_output.mat");
    //let file = std::path::PathBuf::from("./scripts/complex_matlab.mat");
    //let file = std::path::PathBuf::from("./tests/2x2_octave_mirror.mat");

    mat5::parse_file(&file).ok();
}
