use crate::prelude::*;
use nom::bytes::complete as bytes;
use nom::IResult;
use flate2::read::ZlibDecoder;
use std::io::Read;

pub fn parse_file(path: &Path) -> Result<(), Error> {
    let bytes = std::fs::read(path).unwrap();
    dbg!(bytes.len());
    inner(&bytes).unwrap();

    Ok(())
}

macro_rules! make_array_values {
    ($data_type:ty, $num_bytes:expr, $slice:ident, $output:ident) => {
        let mut arr = [0; $num_bytes];

        assert_eq!($slice.get($num_bytes).is_none(), true);

        for i in 0..$num_bytes {
            arr[i] = $slice[i]
        }

        let $output = <$data_type>::from_le_bytes(arr);
    };
}

fn inner(bytes: &[u8]) -> nom::IResult<&[u8], ()> {

    let initial_byte_len = bytes.len();
    let (rest, header) = read_header_information(bytes)?;
    let read_bytes_header = initial_byte_len - rest.len();
    dbg!(&header, read_bytes_header);

    let (mut rest, data_type_initial) = read_overall_datatype_header(rest)?;
    dbg!(&data_type_initial);

    // check for compressed data
    let mut decode_bytes : Vec<u8> = Vec::new();
    if data_type_initial.dtype_integer == 15 {
        println!("data is compressed by zlib - decompressing");
        let mut z = ZlibDecoder::new(rest);
        z.read_to_end(&mut decode_bytes).unwrap();
        println!("decoded bytes length: {}", decode_bytes.len());
        rest = &decode_bytes;

        let (tmp, new_data_type) = read_overall_datatype_header(rest).unwrap();
        rest = tmp;
        dbg!(new_data_type);
    }

    let (rest, array_flags) = read_array_flags(rest).unwrap();
    dbg!(&array_flags);
    println!("flag 1 bits: {:032b}", array_flags.flag_1);
    println!("flag 2 bits: {:032b}", array_flags.flag_2);

    let (rest, dimensions) = read_dimensions(rest).unwrap();
    dbg!(&dimensions);

    let (rest, name) = read_array_name(rest).unwrap();
    dbg!(&name);

    let (rest, data) = read_array_data(rest).unwrap();
    dbg!(&data);

    dbg!(rest);


    Ok((&[], ()))
}

#[derive(Debug)]
struct Header<'a> {
    header_string: String,
    sys_offset_1: u32,
    sys_offset_2: u32,
    sys_offset_1_bytes: &'a [u8],
    sys_offset_2_bytes: &'a [u8],
    endian_marker: String,
}

fn read_header_information(bytes: &[u8]) -> IResult<&[u8], Header> {
    let (rest, header) = bytes::take(116usize)(bytes)?;
    let (rest, sys_offset_1_bytes) = bytes::take(4usize)(rest)?;
    let (rest, sys_offset_2_bytes) = bytes::take(4usize)(rest)?;
    let (rest, version) = bytes::take(2usize)(rest)?;
    let (rest, endian) = bytes::take(2usize)(rest)?;

    make_array_values!(u32, 4, sys_offset_1_bytes, sys_offset_1);
    make_array_values!(u32, 4, sys_offset_2_bytes, sys_offset_2);
    make_array_values!(u16, 2, version, ver_);

    let endian = String::from_utf8(endian.to_vec()).unwrap();

    let header_string = String::from_utf8(header.to_vec()).unwrap();

    let header = Header {
        header_string: header_string.to_string(),
        sys_offset_1,
        sys_offset_2,
        sys_offset_1_bytes,
        sys_offset_2_bytes,
        endian_marker: endian.to_string()
    };

    Ok((rest, header))
}

#[derive(Debug)]
struct DataType <'a> {
    dtype_integer: u32,
    dtype_bytes: &'a [u8],
    dtype_length: u32,
    dtype_length_bytes: &'a [u8]
}

fn read_overall_datatype_header(bytes: &[u8]) -> IResult<&[u8], DataType> {
    let (rest1, first_2_bytes) = bytes::take(2usize)(bytes)?;
    let (_, second_2_bytes) = bytes::take(2usize)(rest1)?;

    dbg!(first_2_bytes, second_2_bytes);

    let (rest, data_type_1_bytes) = bytes::take(4usize)(bytes)?;
    let (rest, data_length_1_bytes) = bytes::take(4usize)(rest)?;

    make_array_values!(u32, 4, data_type_1_bytes, data_type_1);
    make_array_values!(u32, 4, data_length_1_bytes, data_length_1);

    let dtype = DataType {
        dtype_integer: data_type_1,
        dtype_bytes: data_type_1_bytes,
        dtype_length: data_length_1,
        dtype_length_bytes:data_length_1_bytes 
    };
    
    Ok((rest, dtype))
}


#[derive(Debug)]
struct ArrayFlags<'a> {
    array_flag_type: u32,
    array_flag_type_bytes: &'a [u8],
    array_flag_length: u32,
    array_flag_length_bytes: &'a [u8],
    flag_1: u32,
    flag_1_bytes: &'a [u8],
    flag_2: u32,
    flag_2_bytes: &'a [u8]
}

fn read_array_flags (bytes: &[u8]) -> IResult<&[u8], ArrayFlags> {

    let (rest, array_flag_type_bytes) = bytes::take(4usize)(bytes)?;
    let (rest, array_flag_length_bytes) = bytes::take(4usize)(rest)?;
    let (rest, flag_1_bytes) = bytes::take(4usize)(rest)?;
    let (rest, flag_2_bytes) = bytes::take(4usize)(rest)?;

    make_array_values!(u32, 4, array_flag_type_bytes, array_flag_type);
    make_array_values!(u32, 4, array_flag_length_bytes, array_flag_length);
    make_array_values!(u32, 4, flag_1_bytes, flag_1 );
    make_array_values!(u32, 4, flag_2_bytes, flag_2 );

    let flags = ArrayFlags {
        array_flag_type,
        array_flag_type_bytes,
        array_flag_length,
        array_flag_length_bytes,
        flag_1,
        flag_1_bytes,
        flag_2,
        flag_2_bytes,
    };

    Ok((rest, flags))
}

#[derive(Debug)]
struct Dimensions<'a> {
    dimension_type: u32,
    dimension_type_bytes: &'a [u8],
    dimension_length: u32,
    dimension_length_bytes: &'a [u8],
    dimension_sizes: Vec<i32>,
    dimension_sizes_bytes: &'a [u8],
    padding_bytes: usize
}

fn read_dimensions(bytes: &[u8]) -> IResult<&[u8], Dimensions> {
    let (rest, dimension_type_bytes) = bytes::take(4usize)(bytes)?;
    let (rest, dimension_length_bytes) = bytes::take(4usize)(rest)?;

    make_array_values!(u32, 4, dimension_type_bytes, dimension_type);
    make_array_values!(u32, 4, dimension_length_bytes, dimension_length);


    let (rest, dimension_sizes_bytes) = bytes::take(dimension_length as usize)(rest)?;

    let mut dimension_sizes : Vec<i32> = Vec::new();

    for i in 0..(dimension_length/4) {
        let slicer : usize = (i*4) as usize;
        println!("reading one dimension (slicer {slicer})");
        let dim_slice = &dimension_sizes_bytes[slicer..(slicer+4)];
        make_array_values!(i32, 4, dim_slice, dim_n);
        dimension_sizes.push(dim_n);
    }

    let padding_bytes = utils::padding_bytes_required(dimension_length as usize);
    let (rest, _padding_bytes) = bytes::take(padding_bytes)(rest)?;

    let dimension = Dimensions {
        dimension_type,
        dimension_type_bytes,
        dimension_length, 
        dimension_length_bytes,
        dimension_sizes,
        dimension_sizes_bytes,
        padding_bytes
    };

    Ok((rest, dimension))
}

#[derive(Debug)]
struct ArrayName<'a> {
    name_type: u32,
    name_type_bytes: &'a [u8],
    name_length: u32,
    name_length_bytes:&'a [u8],
    name: String,
    name_bytes: &'a [u8],
    padding_bytes: usize
}


fn read_array_name(bytes: &[u8]) -> IResult<&[u8], ArrayName> {
    
    let (rest, name_type_bytes) = bytes::take(4usize)(bytes)?;
    let (rest, name_length_bytes) = bytes::take(4usize)(rest)?;

    make_array_values!(u32, 4, name_length_bytes, name_length);
    make_array_values!(u32, 4, name_type_bytes, name_type);

    let (rest, name_bytes) = bytes::take(name_length as usize)(rest)?;

    let padding_bytes = utils::padding_bytes_required(name_length as usize);
    let (rest, _padding_bytes) = bytes::take(padding_bytes)(rest)?;

    let name = String::from_utf8(name_bytes.to_vec()).unwrap();

    let names = ArrayName {
        name_type,
        name_type_bytes,
        name_length,
        name_length_bytes,
        name,
        name_bytes,
        padding_bytes
    };
    
    Ok((rest, names))
}

#[derive(Debug)]
struct ArrayData<'a> {
    array_type: u32,
    array_type_bytes: &'a [u8],
    array_length: u32,
    array_length_bytes: &'a [u8],
    array_data_bytes: &'a [u8],
    padding_bytes: usize,
}

fn read_array_data(bytes: &[u8]) -> IResult<&[u8], ArrayData> {
    let (rest, array_type_bytes) = bytes::take(4usize)(bytes)?;
    let (rest, array_length_bytes) = bytes::take(4usize)(rest)?;

    make_array_values!(u32, 4, array_type_bytes, array_type);
    make_array_values!(u32, 4, array_length_bytes, array_length);

    let (rest, array_data_bytes) = bytes::take(array_length as usize)(rest)?;

    let padding_bytes = utils::padding_bytes_required(array_length as usize);
    let (rest, _padding_bytes) = bytes::take(padding_bytes)(rest)?;


    let data = ArrayData {
        array_type,
        array_type_bytes,
        array_length,
        array_length_bytes,
        array_data_bytes,
        padding_bytes,
    };

    Ok((rest, data))
}
