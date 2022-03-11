#![doc = include_str!("../README.md")]

mod byte_count;
mod container;
mod num;
mod parse;
mod prelude;
mod utils;
mod write_matrix;

/// Automatically generate an implementation of `mat5::MatFile` for your struct
pub use derive::MatFile;

#[doc(hidden)]
pub use parse::parse_file;
#[doc(hidden)]
pub use utils::fill_byte_padding;

pub use utils::write_default_header;

use prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An io error occured in the operation: `{0}`")]
    Io(#[from] io::Error),
}

/// Orchestrates writing the general structure and individual containers for a given `.mat` file
pub trait MatFile {
    fn write_contents<W: Write>(&self, writer: W) -> Result<(), Error>;
}

/// describes how a container type (vector or matrix) should be serialized to bytes in a `.mat`
/// file
///
/// You probably should not implement this yourself
pub trait Container<T> {
    fn write_container<W: Write>(
        &self,
        writer: W,
        container_name: &'static str,
    ) -> Result<(), Error>;
}

/// Describes numeric types and thier associated matlab
/// identifier constants that can be written to files
///
/// You should not implement this yourself
pub trait Num {
    type LeBytes;

    fn matlab_id() -> u32;

    fn matrix_id() -> u8;

    fn le_bytes(&self) -> Self::LeBytes;
}

/// counds the number of bytes that are required for a matrix
/// uses when writing it to a file
trait ByteCount {
    fn byte_count(&self, array_name: &'static str, n_dim: usize) -> usize;
}

/// describes the order in which a N dimensional matrix's values should be written to the file
pub trait WriteMatrix {
    fn write_matrix<W: Write>(&self, writer: W) -> Result<(), io::Error>;
}

#[doc(hidden)]
pub fn generic_test_runner<T: MatFile>(run_name: &str, check_file_contents: &str, contents: T) {
    let filename = format!("./tests/{run_name}.mat");
    let checker_file = format!("./tests/{run_name}.m");

    let binary_writer = std::fs::File::create(&filename).expect("could not create binary mat file");
    contents.write_contents(binary_writer).unwrap();

    std::fs::write(&checker_file, check_file_contents).expect("could not write octave runner file");

    let mut command = std::process::Command::new("octave");
    command.arg(&checker_file);
    dbg!(&command);

    let out = command.output().unwrap();

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    println!("STDOUT:\n{stdout}\nSTDERR:\n{stderr}");

    if stderr.len() > 0 {
        panic!("stderr was present");
    }

    std::fs::remove_file(filename).unwrap();
    std::fs::remove_file(checker_file).unwrap();
}

#[doc(hidden)]
pub fn check_file_creator(run_name: &str, results_to_load: &[&str], add_semicolon: bool) -> String {
    let filename = format!("./tests/{run_name}.mat");

    let mut out = format!(r#"load("{filename}")"#);

    for result in results_to_load {
        out.push('\n');
        out.push_str(result);

        if add_semicolon {
            out.push(';');
        }
    }

    out
}
