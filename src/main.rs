use mat5::Num;

//let file = std::path::PathBuf::from("./scripts/octave_output.mat");
fn main() {
    let file = std::path::PathBuf::from("./scripts/matlab_output.mat");
    //let file = std::path::PathBuf::from("./scripts/complex_matlab.mat");
    //let file = std::path::PathBuf::from("./tests/2x2_octave_mirror.mat");

    mat5::parse_file(&file).ok();
    dbg!(0b110);
    //000000000000000000000000
    //00000110
    // if complex
    let matlab_class = f64::matrix_id();
    let shift_flags = create_flag_1(matlab_class, true, false, false);
    println!("{:032b}", shift_flags);
    let complex = 0b00000000000000000000100000000110;
    let regular = 0b00000000000000000000000000000110;
    println!("{:032b}", complex);
}

fn create_flag_1(matrix_class: u8, is_complex: bool, is_logical: bool, is_global: bool) -> u32 {
    (matrix_class as u32) ^ (is_complex as u32) << (8 + 3) ^ (is_logical as u32) << (8 + 1) ^ (is_global as u32) << (8 + 2)
}
