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

    let check_file = check_file_creator(runner, &["a", "b", "c"]);
    generic_test_runner(runner, &check_file, item);
}
