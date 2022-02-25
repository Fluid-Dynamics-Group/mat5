mod byte_count;
mod container;
mod num;
mod prelude;
mod utils;
mod write_matrix;

pub use utils::fill_byte_padding;
pub use utils::write_default_header;

use prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An io error occured in the operation: `{0}`")]
    Io(#[from] io::Error),
    #[error("A container name was not specified for the matrix / n-dimensional array")]
    MissingContainerName,
}

/// handles how to write the individual fields
pub trait MatFile {
    fn write_contents<W: Write>(&self, writer: W) -> Result<(), Error>;
}

/// handles writing container types to files, including vectors and matricies
pub trait Container<T> {
    fn write_container<W: Write>(
        &self,
        writer: W,
        container_name: Option<&'static str>,
    ) -> Result<(), Error>;
}

/// Describes numeric types and thier associated matlab
/// identifier constants that can be written to files
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

// verify that our trait system works
#[allow(dead_code)]
fn check() {
    fn inner<T: Container<V>, V: Num>(_: T) {}

    let data = vec![1u32];

    inner(data)
}
