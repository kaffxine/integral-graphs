use std::alloc;
use std::fmt;
use crate::graph::AdjMatrix;

pub struct Matrix {
    n: u64,
    data: *mut i128,
}

pub fn add(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String> {
    if m1.n != m2.n {
        return Err("can't add two matrices of different size".to_string());
    }

    let n = m1.n;
    let m = Matrix::zeroed(n)?;

    for i in 0..n {
        for j in 0..n {
            m.set(i, j, m1.get(i, j)? + m2.get(i, j)?);
        }
    }

    Ok(m)
}

pub fn multiply(m1: &Matrix, m2: &Matrix) -> Result<Matrix, String> {
    if m1.n != m2.n {
        return Err("can't multiply two matrices of different size".to_string());
    }

    let n = m1.n;
    let m = Matrix::zeroed(n)?;

    for i in 0..n {
        for j in 0..n {
            let mut field = 0;
            for k in 0..n {
                field += m1.get(i, k)? * m2.get(k, j)?;
            }
            m.set(i, j, field)?;
        }
    }

    Ok(m)
}

impl Matrix {
    pub fn get_n(&self) -> u64 {
        self.n
    }

    pub fn set(&self, row: u64, column: u64, value: i128) -> Result<(), String> {
        if row < self.n && column < self.n {
            let index = (row as usize) * (self.n as usize) + (column as usize);
            unsafe { *self.data.add(index) = value };
            Ok(())
        } else {
            Err("Matrix::set(): index out of bounds".to_string())
        }
    }

    pub fn get(&self, row: u64, column: u64) -> Result<i128, String> {
        if row < self.n && column < self.n {
            let index = (row as usize) * (self.n as usize) + (column as usize);
            Ok(unsafe { *self.data.add(index) })
        } else {
            Err("Matrix::get(): index out of bounds".to_string())
        }
    }

    pub fn zeroed(n: u64) -> Result<Self, String> {
        let data = unsafe {
            alloc::alloc_zeroed(alloc::Layout::array::<i128>((n as usize) * (n as usize))
                .map_err(|_| "allocation layour error")?) as *mut i128
        };

        if data.is_null() {
            return Err("allocated a null pointer".to_string());
        }

        Ok(Self { n, data })
    }

    pub fn identity(n: u64) -> Result<Self, String> {
        let zeroed = Self::zeroed(n)?;
        for i in 0..n {
            zeroed.set(i, i, 1);
        }
        Ok(zeroed)
    }

    pub fn from_vec_vec(input: Vec<Vec<i128>>) -> Result<Self, String> {
        let n_rows = input.len() as u64;
        let m = Self::zeroed(n_rows)?;
        for (i, row) in input.into_iter().enumerate() {
            let n_cols = row.len();
            if n_cols != n_rows as usize {
                return Err("only square matrices are valid".to_string());
            }
            for (j, val) in row.into_iter().enumerate() {
                m.set(i as u64, j as u64, val)?;
            }
        }
        Ok(m)
    }

    pub fn trace(&self) -> Result<i128, String> {
        let mut result = 0;
        for i in 0..self.n {
            result += self.get(i, i)?;
        }
        Ok(result)
    }
}

impl TryFrom<AdjMatrix> for Matrix {
    type Error = String;

    fn try_from(adjm: AdjMatrix) -> Result<Self, Self::Error> {
        let last_node = adjm.get_last_node();
        let matrix = Self::zeroed(last_node as u64 + 1)?;
        for i in 0..=last_node {
            for j in 0..i {
                if adjm.is_edge(j, i)? {
                    matrix.set(i as u64, j as u64, 1)?;
                    matrix.set(j as u64, i as u64, 1)?;
                }
            }
        }
        return Ok(matrix);
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Matrix({}):\n", self.n)?;
        for i in 0..self.n {
            write!(f, " ")?;
            for j in 0..self.n {
                write!(f, "{} ", self.get(i, j).expect("Matrix fmt::Display"))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.data as *mut u8,
                alloc::Layout::array::<i128>((self.n as usize) * (self.n as usize))
                    .expect("failed to deallocate Matrix")
            )
        }
    }
}
