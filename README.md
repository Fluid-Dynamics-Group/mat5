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
	  [4,5,6]]
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

```
a =

  1  2  3
  4  5  6

b =

   10.000
   11.000
   12.000
   13.500
```
