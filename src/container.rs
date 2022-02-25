use super::prelude::*;
use utils::fill_byte_padding;
use utils::write_array_name;
use utils::write_matrix_dimensions;

const MATRIX_MATLAB_ID: u32 = 14;
const U32_MATLAB_ID: u32 = 6;

impl<T, const BYTES: usize> Container<T> for Vec<T>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write<W: Write>(
        &self,
        mut writer: W,
        _container_name: Option<&'static str>,
    ) -> Result<(), Error> {
        let size = std::mem::size_of::<T>();
        let len = self.len();

        let matlab_id = T::matlab_id();
        writer.write_all(&matlab_id.to_le_bytes())?;

        let byte_length = size * len;
        let byte_length_u32 = byte_length as u32;
        writer.write_all(&byte_length_u32.to_le_bytes())?;

        // then write the data
        self.iter()
            .try_for_each(|x| writer.write_all(&x.le_bytes()))?;

        // pad the rest of the data
        fill_byte_padding(&mut writer, byte_length)?;

        Ok(())
    }
}

// allow references to owned arrays to use the container writing format
impl<T, DIM, const BYTES: usize> Container<T> for &ndarray::Array<T, DIM>
where
    T: Num<LeBytes = [u8; BYTES]>,
    DIM: ndarray::Dimension,
    for<'a> ndarray::ArrayView<'a, T, DIM>: WriteMatrix,
{
    fn write<W: Write>(
        &self,
        writer: W,
        container_name: Option<&'static str>,
    ) -> Result<(), Error> {
        self.view().write(writer, container_name)?;
        Ok(())
    }
}

impl<'a, T, DIM, const BYTES: usize> Container<T> for ndarray::ArrayView<'a, T, DIM>
where
    T: Num<LeBytes = [u8; BYTES]>,
    DIM: ndarray::Dimension,
    Self: WriteMatrix,
{
    fn write<W: Write>(
        &self,
        mut writer: W,
        container_name: Option<&'static str>,
    ) -> Result<(), Error> {
        let container_name = container_name.ok_or(Error::MissingContainerName)?;

        let size = std::mem::size_of::<T>();
        let len = self.len();

        //
        // write the matarix header
        //
        writer.write_all(&MATRIX_MATLAB_ID.to_le_bytes())?;
        // TODO
        let total_matrix_length: usize = self.byte_count(container_name, self.ndim());
        writer.write_all(&total_matrix_length.to_le_bytes())?;

        //
        // array flags
        //
        writer.write_all(&U32_MATLAB_ID.to_le_bytes())?;
        writer.write_all(&8u32.to_le_bytes())?;
        // then  the actual flags
        // zeros with the complex / global / logical values set
        let matrix_class = T::matrix_id() as u64;
        let flags: u64 = 0u64 + (0x00000000 << (5 * 8)) + (matrix_class << (4 * 8));
        writer.write_all(&flags.to_le_bytes())?;

        //
        // array dimensions
        //
        write_matrix_dimensions(&mut writer, self.shape())?;

        //
        // Array name
        //
        write_array_name(&mut writer, container_name)?;

        //
        // Array data (assume non-complex data)
        //
        // header
        let num_matrix_bytes = len * size;
        writer.write_all(&T::matlab_id().le_bytes())?;
        writer.write_all(&(num_matrix_bytes as u32).le_bytes())?;
        // write the actual data
        self.write_matrix(&mut writer)?;
        fill_byte_padding(writer, num_matrix_bytes)?;

        Ok(())
    }
}
