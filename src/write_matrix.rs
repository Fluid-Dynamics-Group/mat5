use crate::prelude;
use ndarray::Dim;
use prelude::Num;
use prelude::WriteMatrix;
use std::io::{self, Write};

impl<'a, T, const BYTES: usize> WriteMatrix for ndarray::ArrayView<'a, T, Dim<[usize; 1]>>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write_matrix<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        let nx = self.dim();

        for i in 0..nx {
            writer.write_all(&self.get(i).unwrap().le_bytes())?;
        }

        Ok(())
    }
}

impl<'a, T, const BYTES: usize> WriteMatrix for ndarray::ArrayView<'a, T, Dim<[usize; 2]>>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write_matrix<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        let (nx, ny) = self.dim();

        for j in 0..ny {
            for i in 0..nx {
                writer.write_all(&self.get((i, j)).unwrap().le_bytes())?;
            }
        }

        Ok(())
    }
}

impl<'a, T, const BYTES: usize> WriteMatrix for ndarray::ArrayView<'a, T, Dim<[usize; 3]>>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write_matrix<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        let (nx, ny, nz) = self.dim();

        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    writer.write_all(&self.get((i, j, k)).unwrap().le_bytes())?;
                }
            }
        }

        Ok(())
    }
}

impl<'a, T, const BYTES: usize> WriteMatrix for ndarray::ArrayView<'a, T, Dim<[usize; 4]>>
where
    T: Num<LeBytes = [u8; BYTES]>,
{
    fn write_matrix<W: Write>(&self, mut writer: W) -> Result<(), io::Error> {
        let (nx, ny, nz, nw) = self.dim();

        for w in 0..nw {
            for k in 0..nz {
                for j in 0..ny {
                    for i in 0..nx {
                        writer.write_all(&self.get((i, j, k, w)).unwrap().le_bytes())?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[test]
/// checks that the bytes writen by Array2<T> are in the correct order
/// that we expect
fn write_arr2() {
    let mut array = ndarray::Array2::<u8>::zeros((2, 2));
    array[[0, 0]] = 1;
    array[[0, 1]] = 2;
    array[[1, 0]] = 3;
    array[[1, 1]] = 4;

    println!("{}", array);

    let expected_order: Vec<u8> = [1u8, 3, 2, 4]
        .into_iter()
        .map(|x| x.le_bytes())
        .flatten()
        .collect();
    let mut buffer = Vec::new();

    array.view().write_matrix(&mut buffer).unwrap();

    assert_eq!(expected_order, buffer);
}
