mod prelude;
mod traits;

use prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("An io error occured in the operation: `{0}`")]
    Io(#[from] io::Error),
    #[error("A container name was not specified for the matrix / n-dimensional array")]
    MissingContainerName
}

pub trait Container<T> {
    fn write<W: Write>(&self, writer: W, container_name: Option<&'static str>) -> Result<(), Error>;
}

pub trait Num {
    type LeBytes;

    fn matlab_id() -> u32;

    fn matrix_id() -> u8;

    fn le_bytes(&self) -> Self::LeBytes;
}

trait ByteCount {
    fn byte_count(&self, array_name: &'static str) -> usize;
}

// verify that our trait system works
#[allow(dead_code)]
fn check() {
    fn inner<T: Container<V>, V: Num>(_: T) {}

    let data = vec![1u32];

    inner(data)
}
