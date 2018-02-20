extern crate rand;
extern crate zero_one;
use vector::Vector;
use self::zero_one::{One, Zero};

use self::rand::Rand;

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    columns: Box<[Vector<T>]>,
}

impl<T: Zero + One + Rand> Matrix<T> {
    pub fn zero(rows: usize, columns: usize) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..columns)
            .map(|_| Vector::from_vec((0usize..rows).map(|_| T::zero()).collect()))
            .collect();

        Matrix {
            columns: columns.into_boxed_slice(),
        }
    }

    pub fn from(columns: Box<[Vector<T>]>) -> Matrix<T> {
        if !columns.is_empty() {
            let len_first = columns[0].len();
            for col in columns.iter() {
                assert_eq!(len_first, col.len(), "All columns must be the same length");
            }
        }
        Matrix { columns }
    }

    pub fn from_vec(columns: Vec<Vector<T>>) -> Matrix<T> {
        Matrix::from(columns.into_boxed_slice())
    }

    pub fn identity(rows: usize, columns: usize) -> Matrix<T> {
        assert_eq!(
            rows, columns,
            "Rows needs to equal columns for Identity matrices"
        );

        let columns: Vec<Vector<T>> = (0..columns)
            .map(|i| {
                Vector::from_vec(
                    (0..rows)
                        .map(|j| if i == j { T::one() } else { T::zero() })
                        .collect(),
                )
            })
            .collect();

        Matrix {
            columns: columns.into_boxed_slice(),
        }
    }

    pub fn random(rows: usize, columns: usize) -> Matrix<T> {
        Matrix::from_function(rows, columns, |_, _| rand::random::<T>())
    }

    pub fn from_function(
        rows: usize,
        columns: usize,
        function: fn(usize, usize) -> T,
    ) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..columns)
            .map(|i| Vector::from_vec((0..rows).map(|j| function(i, j)).collect()))
            .collect();
        Matrix {
            columns: columns.into_boxed_slice(),
        }
    }

    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    pub fn nrows(&self) -> usize {
        if self.ncols() == 0 {
            return 0;
        }
        self.columns[0].len()
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.columns[col][row]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        *(&mut self.columns[col][row]) = value;
    }

    pub fn get_segment(&self, row: usize, col: usize, rows: usize, cols: usize) -> Matrix<&T> {
        assert!(
            row + rows < self.nrows(),
            "Index out of bounds: too many rows"
        );
        assert!(
            col + cols < self.ncols(),
            "Index out of bounds: too many cols"
        );

        Matrix {
            columns: Vec::new().into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ncols_and_rows() {
        let m = Matrix {
            columns: vec![
                Vector::from_vec(vec![1, 2, 3]),
                Vector::from_vec(vec![1, 2, 3]),
                Vector::from_vec(vec![1, 2, 3]),
                Vector::from_vec(vec![1, 2, 3]),
            ].into_boxed_slice(),
        };
        assert_eq!(m.ncols(), 4);
        assert_eq!(m.nrows(), 3);
    }

    #[test]
    fn test_from_vec() {
        let m = Matrix::from_vec(vec![Vector::from_vec(vec![1])]);
        assert_eq!(m.nrows(), 1);
        assert_eq!(m.ncols(), 1);
    }

    #[test]
    fn test_random() {
        let m: Matrix<i32> = Matrix::random(9, 10);
        assert_eq!(m.nrows(), 9);
        assert_eq!(m.ncols(), 10);
    }

    #[test]
    #[should_panic]
    fn test_from_unequal_length() {
        Matrix::from_vec(vec![
            Vector::from_vec(vec![1]),
            Vector::from_vec(vec![1, 2]),
        ]);
    }

    #[test]
    fn identity() {
        let m: Matrix<i32> = Matrix::identity(10, 10);
        // check size
        assert_eq!(10, m.columns.len());
        for i in 0..10 {
            assert_eq!(10, m.columns[i].len());
        }

        assert_eq!(10, (0..10).map(|i| m.columns[i][i]).sum());

        // doesn't work without :i32
        let acc: i32 = (0..10)
            .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
            .sum();
        assert_eq!(10, acc);
    }

    #[test]
    fn zero() {
        let m: Matrix<i32> = Matrix::zero(10, 10);
        // check size
        assert_eq!(10, m.columns.len());
        for i in 0..10 {
            assert_eq!(10, m.columns[i].len());
        }

        // doesn't work without :i32
        let acc: i32 = (0..10)
            .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
            .sum();
        assert_eq!(0, acc);
    }

}
