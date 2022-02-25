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

pub fn fill_byte_padding<W: Write>(mut writer: W, total_bytes: usize) -> Result<(), io::Error> {
    let byte_padding_required = padding_bytes_required(total_bytes);
    dbg!(byte_padding_required);
    for _ in 0..byte_padding_required {
        writer.write_all(&0u8.to_le_bytes())?;
    }

    Ok(())
}

/// fire-and-forget function to write a sane header to the `.mat` file
pub fn write_default_header<W: Write>(mut writer: W) -> Result<(), io::Error> {
    write_text_header(&mut writer, "default text header")?;

    // subsytem information
    writer.write_all(&0u64.le_bytes())?;

    // flag information

    //let version: u16 = 0x0100;
    let version: u16 = 0x0001;
    writer.write_all(&version.le_bytes())?;
    writer.write_all("MI".as_bytes())?;

    Ok(())
}

/// write some text (< 116 bytes) to the header of a file
pub fn write_text_header<W: Write>(mut writer: W, text: &str) -> Result<(), io::Error> {
    writer.write_all(text.as_bytes())?;

    let remaining_bytes = 116 - text.as_bytes().len();

    for _ in 0..remaining_bytes {
        writer.write_all(&0u8.le_bytes())?;
    }

    Ok(())
}

pub(crate) fn padding_bytes_required(total_bytes: usize) -> usize {
    total_bytes % 8
}

#[test]
fn check_default_header_len() {
    let mut buffer = Vec::new();
    write_default_header(&mut buffer).unwrap();
    assert_eq!(buffer.len(), 128);
}
