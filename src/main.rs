use mat5::Container;
use mat5::MatFile;
use mat5::Num;
use std::io::Write;

struct Foo {
    inner: ndarray::Array2<u32>,
}

struct Bar{
    inner: Vec<u32>
}

struct Single {
    inner: u32,
}


impl Foo {
    fn new() -> Self {
        let mut inner = ndarray::Array2::zeros((2, 2));
        inner[[0, 0]] = 1;
        inner[[0, 1]] = 2;
        inner[[1, 0]] = 3;
        inner[[1, 1]] = 4;

        Foo { inner }
    }
}

impl Bar {
    fn new() -> Self {

        Bar { inner: vec![1,2,3,4, 5] }
    }
}

impl MatFile for Foo {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;

        println!("writing container");
        mat5::Container::write_container(&self.inner, &mut writer, Some("my_array"))?;

        Ok(())
    }
}

impl MatFile for Bar {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;

        println!("writing container");
        mat5::Container::write_container(&self.inner, &mut writer, None)?;

        Ok(())
    }
}

impl MatFile for Single {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;

        println!("writing container");
        dbg!(u32::matlab_id());
        writer.write_all(&u32::matlab_id().le_bytes())?;
        writer.write_all(&8u32.le_bytes())?;
        writer.write_all(&self.inner.le_bytes())?;

        mat5::fill_byte_padding(&mut writer, 4)?;

        //mat5::Container::write_container(&self.inner, &mut writer, None)?;

        Ok(())
    }
}
fn main() {
    //let foo = Single { inner: 10 };
    let foo = Foo::new();
    let mut file = std::fs::File::create("./scripts/test_file.mat").unwrap();
    foo.write_contents(&mut file).unwrap();

    //let file = std::path::PathBuf::from("./scripts/octave_output.mat");
    let file = std::path::PathBuf::from("./scripts/test_file.mat");
    mat5::parse_file(&file).ok();

    dbg!(0b1101);
    dbg!(0b11100110);

    //dbg!(0x10000000000u64);
}
