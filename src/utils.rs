use crate::prelude::*;

pub(crate) fn write_array_name<W: Write>(mut writer: W, array_name: &str) -> Result<(), io::Error> {
    writer.write_all(&i8::matlab_id().le_bytes())?;

    let num_bytes = array_name.as_bytes().len();
    let bytes_u32: u32 = num_bytes as u32;
    writer.write_all(&bytes_u32.le_bytes())?;

    writer.write_all(&array_name.as_bytes())?;
    fill_byte_padding(writer, num_bytes)?;

    Ok(())
}

pub(crate) fn write_matrix_dimensions<W: Write>(
    mut writer: W,
    dimension: &[usize],
) -> Result<(), io::Error> {
    // first specify that we are writing
    writer.write_all(&i32::matlab_id().le_bytes())?;

    let bytes_written = std::mem::size_of::<i32>() * dimension.len();
    writer.write_all(&(bytes_written as i32).le_bytes())?;

    for dim in dimension {
        let dim = *dim as i32;
        writer.write_all(&dim.le_bytes())?;
    }

    fill_byte_padding(writer, bytes_written)?;

    Ok(())
}

pub fn fill_byte_padding<W: Write>(mut writer: W, total_bytes: usize) -> Result<(), io::Error> {
    let byte_padding_required = padding_bytes_required(total_bytes);
    for _ in 0..byte_padding_required {
        writer.write_all(&0u8.le_bytes())?;
    }

    Ok(())
}

/// fire-and-forget function to write a sane header to the `.mat` file
pub fn write_default_header<W: Write>(mut writer: W) -> Result<(), io::Error> {
    write_text_header(&mut writer, "default text header")?;

    // subsytem information
    writer.write_all(&0u32.le_bytes())?;
    writer.write_all(&0u32.le_bytes())?;

    // flag information
    let version: i16 = 0x0100;
    writer.write_all(&version.le_bytes())?;
    // for some reason this indicates LE bytes (???)
    writer.write_all("IM".as_bytes())?;

    Ok(())
}

/// write some text (< 116 bytes) to the header of a file
pub fn write_text_header<W: Write>(mut writer: W, text: &str) -> Result<(), io::Error> {
    writer.write_all(text.as_bytes())?;

    let remaining_bytes = 116 - text.as_bytes().len();

    for _ in 0..remaining_bytes {
        writer.write_all(&32u8.le_bytes())?;
    }

    Ok(())
}

/// calculate the number of bytes that should be inserted to make the array
/// end on a 64 bit boundary
pub(crate) fn padding_bytes_required(total_bytes: usize) -> usize {
    let bytes_over_ending = total_bytes % 8;

    if bytes_over_ending != 0 {
        8 - bytes_over_ending
    } else {
        0
    }
}

/// create the first array flag through bit shifting
pub(crate) fn create_flag_1(
    matrix_class: u8,
    is_complex: bool,
    is_logical: bool,
    is_global: bool,
) -> u32 {
    (matrix_class as u32)
        ^ (is_complex as u32) << (8 + 3)
        ^ (is_logical as u32) << (8 + 1)
        ^ (is_global as u32) << (8 + 2)
}

#[test]
fn check_default_header_len() {
    let mut buffer = Vec::new();
    write_default_header(&mut buffer).unwrap();
    assert_eq!(buffer.len(), 128);
}

#[test]
fn array_name_length_1() {
    let array_name = "123456789";
    let mut buffer = Vec::new();
    assert!(array_name.len() > 8);
    assert!(array_name.len() < 16);
    write_array_name(&mut buffer, array_name).unwrap();

    // 8 for header, 16 from the name (rounded up from 9 with 7 padding bytes)
    assert_eq!(buffer.len(), 8 + 16);
}
