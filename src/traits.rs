use super::prelude::*;

const MATRIX_MATLAB_ID: u32 = 14;
const U32_MATLAB_ID: u32 = 6;

//impl <T, const V: usize> Container<T> for Vec<T>
impl<T, const BYTES: usize> Container<T> for Vec<T>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write<W: Write>(&self, mut writer: W, _container_name: Option<&'static str>) -> Result<(), Error> {
        let size = std::mem::size_of::<T>();
        let len = self.len();

        let matlab_id = T::matlab_id();
        writer.write_all(&matlab_id.to_le_bytes())?;

        let byte_length = size * len;
        let byte_length_u32 =byte_length as u32;
        writer.write_all(&byte_length_u32.to_le_bytes())?;

        // then write the data
        self.iter().try_for_each(|x| writer.write_all(&x.le_bytes()))?;

        // pad the rest of the data
        fill_byte_padding(&mut writer, byte_length)?;

        Ok(())
    }
}

impl<T, const BYTES: usize> Container<T> for ndarray::Array3<T>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write<W: Write>(&self, mut writer: W, container_name: Option<&'static str>) -> Result<(), Error> {
        let size = std::mem::size_of::<T>();
        let len = self.len();

        // 
        // write the matarix header
        //
        writer.write_all(&MATRIX_MATLAB_ID.to_le_bytes())?;
        // TODO
        let total_matrix_length: usize = 0;
        writer.write_all(&total_matrix_length.to_le_bytes())?;

        // 
        // array flags
        //
        writer.write_all(&U32_MATLAB_ID.to_le_bytes())?;
        writer.write_all(&8u32.to_le_bytes())?;
        // then  the actual flags
        // zeros with the complex / global / logical values set
        let matrix_class = T::matrix_id() as u64;
        let flags: u64 = 0u64 + (0x00000000 << (5 * 8)) + (matrix_class << (4*8));
        writer.write_all(&flags.to_le_bytes())?;

        // 
        // array dimensions
        //
        write_matrix_dimensions(&mut writer, self.shape())?;

        //
        // Array name
        //
        write_array_name(&mut writer, container_name.ok_or(Error::MissingContainerName)?)?;

        //
        // Array data (assume non-complex data)
        //
        // header
        let num_matrix_bytes = len * size;
        writer.write_all(&T::matlab_id().le_bytes())?;
        writer.write_all(&(num_matrix_bytes as u32).le_bytes())?;
        // write the actual data
        // TODO: check how this is written, we probably want to do raw indexing 
        // through a trait to avoid transpose issues
        self.iter().try_for_each(|x| writer.write_all(&x.le_bytes()))?;
        fill_byte_padding(writer, num_matrix_bytes)?;

        Ok(())
    }
}

impl <'a, T, DIM> ByteCount for ndarray::ArrayView<'a, T, DIM> {
    fn byte_count(&self, array_name: &'static str) -> usize {
        //
        todo!()
    }
}

fn write_array_name<W: Write>(mut writer: W, array_name: &str) -> Result<(), io::Error> {
    writer.write_all(&u8::matlab_id().le_bytes())?;

    let num_bytes = array_name.as_bytes().len();
    let bytes_u32 : u32 = num_bytes as u32;
    writer.write_all(&bytes_u32.le_bytes())?;

    writer.write_all(array_name.as_bytes())?;
    fill_byte_padding(writer, num_bytes)?;

    Ok(())
}

fn write_matrix_dimensions<W: Write>(mut writer: W, dimension: &[usize]) -> Result<(), io::Error> {
    // first specify that we are writing 
    writer.write_all(&u32::matlab_id().le_bytes())?;

    for dim in dimension {
        let dim = *dim as u32;
        writer.write_all(&dim.le_bytes())?;
    }

    let bytes_written = std::mem::size_of::<u32>() * dimension.len();
    fill_byte_padding(writer, bytes_written)?;

    Ok(())
}

fn fill_byte_padding<W: Write>(mut writer: W, total_bytes: usize) -> Result<(), io::Error> {
    let byte_padding_required = buffer_bytes_required(total_bytes);
    for _ in 0..byte_padding_required {
        writer.write_all(&0u8.to_le_bytes())?;
    }

    Ok(())
}

fn buffer_bytes_required(total_bytes: usize) -> usize {
    total_bytes % 8
}

/// helper macro to implement the Num trait to a variety of numerical types
///
/// `$target_type` - the numerical type literal we are implementing on
/// `$matlab_id`   - u32 value from mat specification that this type is represented by
/// `$num_bytes`   - how many bytes are contained in the value
macro_rules! impl_num {
    ($($target_type:ty,$matlab_id:expr, $matrix_id: expr, $num_bytes:expr),+)=> {
        $(
            impl crate::Num for $target_type {
                type LeBytes = [u8;$num_bytes];

                fn matlab_id() -> u32 {
                    $matlab_id
                }

                fn matrix_id() -> u8 {
                    $matrix_id
                }

                fn le_bytes(&self) -> Self::LeBytes {
                    self.to_le_bytes()
                }
            }
        )+
    }
}

// impl the matlab datatype extensions for the following types
impl_num!{
    i8,  1, 8, 1,
    u8,  2, 9, 1,
    i16, 3, 10, 2,
    u16, 4, 11, 2,
    i32, 5, 12, 4,
    u32, 6, 13, 4,
    f32, 7, 6, 4,
    f64, 9, 7, 8,
    i64, 12, 14, 8,
    u64, 13, 15, 8
}
