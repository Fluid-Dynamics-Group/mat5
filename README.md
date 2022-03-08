# mat5

mat5 is a proc-macro-based library for writing rust arrays to level 5 Mat-Files (`.mat` files).

* [ ] Loading .mat files
  * [ ] Numeric arrays
  * [ ] Cell arrays
  * [ ] Structure arrays
  * [ ] Object arrays
  * [ ] Character arrays
  * [ ] Sparse arrays
* [ ] Writing .mat files
  * [x] Numeric arrays
    * [x] Non Complex Types
    * [ ] Complex Types
  * [ ] Cell arrays
  * [ ] Structure arrays
  * [ ] Object arrays
  * [ ] Character arrays
  * [ ] Sparse arrays

You can find information on the specification from MathWorks
[here](https://www.mathworks.com/help/pdf_doc/matlab/matfile_format.pdf), 
although, there is also a similar unofficial document with better figures 
[here](https://data.cresis.ku.edu/data/mat_reader/matfile_format.pdf)


## Usage

The easiest way to get started writing files is through the proc macro interface. 
If you want to write two arrays of numeric data `a` and `b`, then you can do the following:


```rust
use mat5::MatFile;
use ndarray::{Array2, Array1};
use ndarray::{arr2, arr1};

#[derive(mat5::MatFile)]
struct Foo {
    a: Array2<u64>,
    b: Array1<f32>,
}

// create a two dimensional array
let a = ndarray::arr2(
	&[[1, 2, 3],
	  [4, 5, 6]]
);

// create a one dimensional array
let b = ndarray::arr1(&[10., 11., 12., 13.5]);

let foo = Foo { a, b };

let mut file = std::fs::File::create("./tests/my_file.mat").unwrap();
foo.write_contents(&mut file);
```

Then, you can trivially load the files using matlab:

```matlab
load("./tests/my_file.mat")

% print the contents of each of the files
a
b
```

which outputs:

```matlab
a =

  1  2  3
  4  5  6

b =

   10.000
   11.000
   12.000
   13.500
```

The following code was automatically generated:

```rust,ignore
impl mat5::MatFile for Foo {
    fn write_contents<W: std::io::Write>(&self, mut writer: W) -> Result<(), mat5::Error> {
        mat5::write_default_header(&mut writer)?;
        mat5::Container::write_container(&self.a, &mut writer, "a")?;
        mat5::Container::write_container(&self.b, &mut writer, "b")?;
        Ok(())
    }
}
```

## Wrapper Types

If your type wraps a type that implements the [`Container`](`crate::Container`) trait,
you can use the `#[mat5(deref)]` attribute to tell `mat5` to call [`Deref`](`std::ops::Deref`) on the 
type before serializing it to the file:

```rust
#[derive(mat5::MatFile)]
struct Foo {
    #[mat5(deref)]
    some_array: WrapArray,
}

// `WrapArray` is just a proof of concent wrapper 
// type around a container that implements `Container`
struct WrapArray(Array2<u64>);

impl std::ops::Deref for Wrap {
    type Target = Array2<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```


