use mat5::check_file_creator;
use mat5::generic_test_runner;
use mat5::Container;
use mat5::MatFile;
use mat5::Num;
use std::io::Write;

use ndarray::Array1;
use ndarray::Array2;

struct PaddingName {
    inner: Array2<u32>,
}

impl MatFile for PaddingName {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;
        self.inner
            .view()
            .write_container(&mut writer, "a12345678")?;
        Ok(())
    }
}

#[cfg(test)]
fn create_octave_runner(mat_file_name: &str, array_name: &str, load_array: bool) -> String {
    if load_array {
        format!(
            r#"
            load("{mat_file_name}")

            {array_name}
        "#
        )
    } else {
        format!(
            r#"
            load("{mat_file_name}")
            disp('file loaded successfully')
        "#
        )
    }
}

#[test]
fn name_requires_padding() {
    let run = "requires_padding";

    // write the binary .mat file
    let padding = PaddingName {
        inner: Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap(),
    };

    let check_file = check_file_creator(run, &["a12345678"], false);
    generic_test_runner(run, &check_file, padding);
}

struct PaddingArray {
    inner: Array2<u8>,
}

impl MatFile for PaddingArray {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;
        self.inner.view().write_container(&mut writer, "a1234567")?;
        Ok(())
    }
}

#[test]
// both the array bytes adn teh
fn array_bytes_require_padding() {
    let run = "array_require_padding";

    // write the binary .mat file
    let padding = PaddingArray {
        inner: Array2::from_shape_vec((2, 2), vec![1, 2, 3, 4]).unwrap(),
    };

    let check_file = check_file_creator(run, &["a1234567"], false);
    generic_test_runner(run, &check_file, padding);
}

struct PadDimensions {
    inner: Array1<u32>,
}

impl MatFile for PadDimensions {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;
        self.inner.view().write_container(&mut writer, "a1234567")?;
        Ok(())
    }
}

#[test]
// The dimensions of the array are required to be padded correctly
fn dimensions_require_padding() {
    let run = "pad_dimensions";

    // write the binary .mat file
    let padding = PadDimensions {
        inner: Array1::from_shape_vec(2, vec![1, 2]).unwrap(),
    };

    let check_file = check_file_creator(run, &["a1234567"], false);
    generic_test_runner(run, &check_file, padding);
}

struct OneItem<T> {
    inner: Array1<T>,
}

impl<T> OneItem<T>
where
    T: Default + std::ops::Add<T, Output = T> + std::convert::TryFrom<u8>,
    <T as TryFrom<u8>>::Error: std::fmt::Debug,
{
    fn new() -> Self {
        OneItem {
            inner: Array1::from_shape_vec(
                3,
                vec![
                    1u8.try_into().unwrap(),
                    2u8.try_into().unwrap(),
                    3u8.try_into().unwrap(),
                ],
            )
            .unwrap(),
        }
    }
}

impl<T, const BYTES: usize> MatFile for OneItem<T>
where
    T: mat5::Num<LeBytes = [u8; BYTES]>,
    for<'a> ndarray::ArrayView1<'a, T>: mat5::WriteMatrix,
{
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;

        self.inner.view().write_container(&mut writer, "a1234567")?;
        Ok(())
    }
}

macro_rules! make_type_test {
    ($($test_name:ident, $type:ty),+) => {

        $(
            #[test]
            // The dimensions of the array are required to be padded correctly
            fn $test_name() {
                let run = stringify!($test_name);
                let item = OneItem::<$type>::new();

                let check_file = check_file_creator(run, &["a1234567"], false);
                generic_test_runner(run, &check_file, item);
            }
        )+

    }
}

make_type_test!(
    one_item_u8,
    u8,
    one_item_i8,
    i8,
    one_item_u16,
    u16,
    one_item_i16,
    i16,
    one_item_i32,
    i32,
    one_item_u32,
    u32,
    one_item_f32,
    f32,
    one_item_f64,
    f64,
    one_item_i64,
    i64,
    one_item_u64,
    u64
);

struct EmptyFile;

impl MatFile for EmptyFile {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;

        Ok(())
    }
}

#[test]
fn empty_file() {
    let run = "empty_file";
    let item = EmptyFile;

    let check_file = check_file_creator(run, &[], false);
    generic_test_runner(run, &check_file, item);
}

struct OctaveMirrorDebug {
    inner: Array2<f64>,
}

impl MatFile for OctaveMirrorDebug {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;
        self.inner.view().write_container(&mut writer, "mat")?;

        Ok(())
    }
}

#[test]
// used for debugging against matlab / octave files
fn two_by_two_mirror_octave_test() {
    let run = "2x2_octave_mirror";
    let item = OctaveMirrorDebug {
        inner: ndarray::arr2(&[[1., 2.], [3., 4.]]),
    };
    dbg!(&item.inner);

    let check_file = check_file_creator(run, &["mat"], false);
    generic_test_runner(run, &check_file, item);
}

struct MultiArrayCheck {
    inner1: Array2<f64>,
    inner2: Array2<f64>,
}

impl MatFile for MultiArrayCheck {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;
        self.inner1.view().write_container(&mut writer, "mat1")?;
        self.inner2.view().write_container(&mut writer, "mat2")?;

        Ok(())
    }
}

#[test]
fn multi_array_per_file() {
    let run = "multi_array_file";
    let item = MultiArrayCheck {
        inner1: ndarray::arr2(&[[1., 2.], [3., 4.]]),
        inner2: ndarray::arr2(&[[1., 2.], [3., 4.]]),
    };
    let check_file = check_file_creator(run, &["mat1", "mat2"], false);
    generic_test_runner(run, &check_file, item);
}
