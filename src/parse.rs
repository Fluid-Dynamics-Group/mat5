use crate::prelude::*;
use nom::bytes::complete as bytes;

pub fn parse_file(path: &Path) -> Result<(), Error> {
    let bytes = std::fs::read(path).unwrap();
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
    }
}

fn inner(bytes: &[u8]) -> nom::IResult<&[u8], ()> {
    dbg!(bytes);
    let len : usize= 116;
    let (rest, header) = bytes::take(len)(bytes)?;
    let (rest, subsys_offset_1) = bytes::take(4usize)(rest)?;
    let (rest, subsys_offset_2) = bytes::take(4usize)(rest)?;
    let (rest, version) = bytes::take(2usize)(rest)?;
    let (rest, endian) = bytes::take(2usize)(rest)?;
    let (rest, data_type_1) = bytes::take(4usize)(rest)?;
    let (rest, data_length_1) = bytes::take(4usize)(rest)?;

    // in the array
    let (rest, array_flag_type) = bytes::take(4usize)(rest)?;
    let (rest, array_flag_length) = bytes::take(4usize)(rest)?;
    let (rest, arr_flag) = bytes::take(8usize)(rest)?;

    // start dimensions
    let (rest, dimension_type) = bytes::take(4usize)(rest)?;
    let (rest, dimension_length) = bytes::take(4usize)(rest)?;


    make_array_values!(u32, 4, data_length_1, data_length_1);

    let header_text = String::from_utf8_lossy(header);

    dbg!(header_text);

    make_array_values!(u32, 4, subsys_offset_1, sys_offset_1);
    dbg!(subsys_offset_1, sys_offset_1);

    make_array_values!(u32, 4, subsys_offset_2, sys_offset_2);
    dbg!(subsys_offset_2, sys_offset_2);
    //dbg!(u32::from_le_bytes(subsys_offset_1));

    make_array_values!(u16, 2, version, ver_);
    dbg!(version, ver_);

    let endian = String::from_utf8_lossy(endian);
    dbg!(endian);

    make_array_values!(u32, 4, data_type_1, data_type_1);
    dbg!(data_type_1);
    dbg!(data_length_1);


    // array flag printing
    make_array_values!(u32, 4, array_flag_type, array_flag_type);
    dbg!(array_flag_type);

    make_array_values!(u32, 4, array_flag_length, array_flag_length_);
    dbg!(array_flag_length_);

    dbg!(arr_flag);
    make_array_values!(u64, 8, arr_flag, arr_flag);
    println!("array flags: {:064b}", arr_flag);

    //Ok(((), ()));
    Ok((&[], ()))
}
