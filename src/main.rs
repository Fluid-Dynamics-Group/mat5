use mat5::Container;
use mat5::MatFile;
use mat5::Num;
use std::io::Write;

struct Foo {
    inner: ndarray::Array2<u32>,
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

impl MatFile for Foo {
    fn write_contents<W: Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;

        println!("writing container");
        mat5::Container::write_container(&self.inner, &mut writer, Some("my_array"))?;

        Ok(())
    }
}

fn main() {
    let foo: Foo = Foo::new();
    let mut file = std::fs::File::create("./scripts/test_file.mat").unwrap();
    foo.write_contents(&mut file).unwrap();

    let num : u8 = u32::matrix_id();
    let shift = 0u64 + (num as u64) << (4*8);
    println!("{:b}", num);
    println!("after shifting");
    println!("{:b}", shift);

}
