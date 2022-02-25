use crate::prelude::*;

pub(crate) fn write_array_name<W: Write>(mut writer: W, array_name: &str) -> Result<(), io::Error> {
    writer.write_all(&u8::matlab_id().le_bytes())?;

    let num_bytes = array_name.as_bytes().len();
    let bytes_u32: u32 = num_bytes as u32;
    writer.write_all(&bytes_u32.le_bytes())?;

    writer.write_all(array_name.as_bytes())?;
    fill_byte_padding(writer, num_bytes)?;

    Ok(())
}

pub(crate) fn write_matrix_dimensions<W: Write>(
    mut writer: W,
    dimension: &[usize],
) -> Result<(), io::Error> {
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

pub(crate) fn fill_byte_padding<W: Write>(
    mut writer: W,
    total_bytes: usize,
) -> Result<(), io::Error> {
    let byte_padding_required = padding_bytes_required(total_bytes);
    for _ in 0..byte_padding_required {
        writer.write_all(&0u8.to_le_bytes())?;
    }

    Ok(())
}

pub(crate) fn padding_bytes_required(total_bytes: usize) -> usize {
    total_bytes % 8
}
