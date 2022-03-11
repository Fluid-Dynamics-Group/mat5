use mat5::check_file_creator;
use mat5::generic_test_runner;

#[derive(mat5::MatFile)]
struct Foo {
    a: ndarray::Array2<u64>,
    b: ndarray::Array3<u8>,
    c: ndarray::Array4<f32>,
}

#[test]
fn derived_proc_macro() {
    let runner = "derived_proc_macro";
    let item = Foo {
        a: ndarray::Array2::zeros((10, 10)),
        b: ndarray::Array3::zeros((10, 10, 2)),
        c: ndarray::Array4::zeros((10, 10, 1, 5)),
    };

    let check_file = check_file_creator(runner, &["a", "b", "c"], false);
    generic_test_runner(runner, &check_file, item);
}

#[derive(mat5::MatFile)]
struct Vector {
    a: Vec<f64>,
}

#[test]
fn derived_vector() {
    let runner = "derived_vector";
    let item = Vector {
        a: vec![1., 2., 3., 4.],
    };

    let check_file = check_file_creator(runner, &["a"], false);
    generic_test_runner(runner, &check_file, item);
}

#[derive(mat5::MatFile)]
struct Slice<'a> {
    a: &'a [u8],
}

#[test]
fn derived_slice() {
    let runner = "derived_slice";
    let vec = vec![1, 2, 3, 4];
    let item = Slice { a: vec.as_slice() };

    let check_file = check_file_creator(runner, &["a"], false);
    generic_test_runner(runner, &check_file, item);
}

#[derive(mat5::MatFile)]
struct Probe {
    viscous: ndarray::Array3<f64>,
    log_law: ndarray::Array3<f64>,
    freestream: ndarray::Array3<f64>,
}

#[test]
fn probe_data() {
    let nx = 100;
    let ny =  4;
    let nz = 150;
    let shape = (nx, ny, nz);
    let runner = "probe_data";
    let arr = ndarray::Array1::linspace(1., (nx * ny * nz) as f64, nx * ny * nz).into_shape(shape).unwrap();
    dbg!(&arr);

    let item = Probe { 
        viscous: arr.clone(),
        log_law: arr.clone(),
        freestream: arr.clone(),
    };

    let check_file = check_file_creator(runner, &["viscous", "log_law", "freestream"], true);
    generic_test_runner(runner, &check_file, item);
    panic!()
}
