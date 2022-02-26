/// helper macro to implement the Num trait to a variety of numerical types
///
/// `$target_type` - the numerical type literal we are implementing on
/// `$matlab_id`   - u32 value from mat specification that this type is represented by
/// `$num_bytes`   - how many bytes are contained in the value
macro_rules! impl_num {
    ($($target_type:ty, $matlab_id:expr, $matrix_id: expr, $num_bytes:expr),+)=> {
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
impl_num! {
    i8,  1,  8,  1,
    u8,  2,  9,  1,
    i16, 3,  10, 2,
    u16, 4,  11, 2,
    i32, 5,  12, 4,
    u32, 6,  13, 4,
    f32, 7,  7,  4,
    f64, 9,  6,  8,
    i64, 12, 14, 8,
    u64, 13, 15, 8
}
