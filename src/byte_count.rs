use super::prelude::*;
use utils::padding_bytes_required;

impl<'a, T, DIM> ByteCount for ndarray::ArrayView<'a, T, DIM>
where
    DIM: ndarray::Dimension,
{
    fn byte_count(&self, array_name: &'static str, n_dim: usize) -> usize {
        let mut bytes = 0;

        //
        // array flags header
        //
        bytes += 8;
        bytes += 8;

        //
        // array dimensions
        //

        // data type
        let _tmp = bytes;
        bytes += 8;
        // array dimension information is always u32 in type
        let bytes_for_array_dimensions = std::mem::size_of::<u32>() * n_dim;
        let padding_bytes = padding_bytes_required(bytes_for_array_dimensions);
        bytes += bytes_for_array_dimensions + padding_bytes;

        //
        // Array name
        //

        // data type header / tag
        let _tmp = bytes;
        bytes += 8;
        // the bytes required for the characters
        let array_name_bytes = array_name.bytes().len();
        let padding_bytes_name = padding_bytes_required(array_name_bytes);
        bytes += array_name_bytes + padding_bytes_name;

        //
        // Numeric data
        //

        let _tmp = bytes;
        bytes += 8;
        let array_bytes = self.len() * std::mem::size_of::<T>();
        let array_padding = padding_bytes_required(array_bytes);
        bytes += array_bytes + array_padding;

        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::ByteCount;

    #[test]
    // check that the calculation for the number of bytes that a matrix
    // will consume when writing to the buffer is correct.
    //
    // 128 bytes is pulled from figure 1-7 in the specification. Since
    // we are not writing complex data, we remove the last 5 rows
    // of bytes from the overal value (5*8)
    fn array_number_bytes_u64() {
        let array = ndarray::Array2::<u64>::zeros((2, 2));

        let array_bytes = 128 - (8 * 5);
        crate::ByteCount::byte_count(&array.view(), "my_array", 2);
        assert_eq!(array_bytes, array.view().byte_count("my_array", 2));
    }

    #[test]
    fn array_number_bytes_u32() {
        let array = ndarray::Array2::<u32>::zeros((2, 2));

        let array_bytes = 9 * 8;
        crate::ByteCount::byte_count(&array.view(), "my_array", 2);
        assert_eq!(array_bytes, array.view().byte_count("my_array", 2));
    }
}
