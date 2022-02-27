use super::prelude::*;
use utils::fill_byte_padding;
use utils::write_array_name;
use utils::write_matrix_dimensions;

const MATRIX_MATLAB_ID: u32 = 14;

impl<T, const BYTES: usize> Container<T> for Vec<T>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write_container<W: Write>(
        &self,
        mut writer: W,
        _container_name: Option<&'static str>,
    ) -> Result<(), Error> {
        let size = std::mem::size_of::<T>();
        let len = self.len();

        let matlab_id = T::matlab_id();
        writer.write_all(&matlab_id.le_bytes())?;

        dbg!(std::mem::size_of_val(&matlab_id) + std::mem::size_of_val(&matlab_id.le_bytes()));

        let byte_length = size * len;
        let byte_length_u32 = byte_length as u32;
        writer.write_all(&byte_length_u32.le_bytes())?;

        // then write the data
        self.iter()
            .try_for_each(|x| writer.write_all(&x.le_bytes()))?;

        // pad the rest of the data
        fill_byte_padding(&mut writer, byte_length)?;

        Ok(())
    }
}

// allow references to owned arrays to use the container writing format
impl<T, DIM, const BYTES: usize> Container<T> for ndarray::Array<T, DIM>
where
    T: Num<LeBytes = [u8; BYTES]>,
    DIM: ndarray::Dimension,
    for<'a> ndarray::ArrayView<'a, T, DIM>: WriteMatrix,
{
    fn write_container<W: Write>(
        &self,
        writer: W,
        container_name: Option<&'static str>,
    ) -> Result<(), Error> {
        self.view().write_container(writer, container_name)?;
        Ok(())
    }
}

impl<'a, T, DIM, const BYTES: usize> Container<T> for ndarray::ArrayView<'a, T, DIM>
where
    T: Num<LeBytes = [u8; BYTES]>,
    DIM: ndarray::Dimension,
    Self: WriteMatrix,
{
    fn write_container<W: Write>(
        &self,
        mut writer: W,
        container_name: Option<&'static str>,
    ) -> Result<(), Error> {
        let container_name = container_name.ok_or(Error::MissingContainerName)?;

        //
        // write the matarix header
        //
        writer.write_all(&MATRIX_MATLAB_ID.le_bytes())?;
        let total_matrix_length: usize = self.byte_count(container_name, self.ndim());
        dbg!(total_matrix_length);
        writer.write_all(&(total_matrix_length as u32).le_bytes())?;

        println!("finish matrix header");

        //
        // array flags
        //
        writer.write_all(&u32::matlab_id().le_bytes())?;
        writer.write_all(&8u32.le_bytes())?;

        let flag_1= utils::create_flag_1(T::matrix_id(), false, false, false);
        writer.write_all(&flag_1.le_bytes())?;
        writer.write_all(&0u32.le_bytes())?;

        //
        // array dimensions
        //
        write_matrix_dimensions(&mut writer, self.shape())?;

        println!("finish matrix dimensions");

        //
        // Array name
        //
        write_array_name(&mut writer, container_name)?;

        println!("finish array name");

        //
        // Array data (assume non-complex data)
        //
        // header
        let num_matrix_bytes = self.len() * std::mem::size_of::<T>();
        writer.write_all(&dbg!(T::matlab_id()).le_bytes())?;
        writer.write_all(&(num_matrix_bytes as u32).le_bytes())?;
        // write the actual data
        self.write_matrix(&mut writer)?;
        println!("padding matrix bytes");
        fill_byte_padding(writer, num_matrix_bytes)?;

        Ok(())
    }
}

#[test]
fn container_serialization_arr2_u64() {
    let array = ndarray::Array2::<u64>::zeros((2, 2));

    println!("{}", array);

    // comes from figure 1-7
    // 128 bytes of matrix content + the first 8 bytes for the header
    //
    // since we are not writing complex data, skip the last 5 rows (8 bytes each)
    let expected_len = 128 + 8 - (5 * 8);

    let mut buffer = Vec::new();

    array
        .view()
        .write_container(&mut buffer, Some("my_array"))
        .unwrap();

    assert_eq!(expected_len, buffer.len());
}

#[test]
fn container_serialization_arr2_f32() {
    let array = ndarray::Array2::<f32>::zeros((2, 2));

    println!("{}", array);

    // comes from figure 1-7, adapted for 32 bit matrix data
    let expected_len = 10 * 8;

    let mut buffer = Vec::new();

    array
        .view()
        .write_container(&mut buffer, Some("my_array"))
        .unwrap();

    assert_eq!(expected_len, buffer.len());
}
