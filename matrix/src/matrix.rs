extern crate rand;
extern crate zero_one;
use vector::Vector;
use self::zero_one::{One, Zero};

use self::rand::Rand;

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    columns: Vec<Vector<T>>,
}

impl<T: Zero + One + Rand> Matrix<T> {
    pub fn zero(rows: usize, columns: usize) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..columns)
            .map(|_| Vector::new((0usize..rows).map(|_| T::zero()).collect()))
            .collect();

        Matrix { columns }
    }

    pub fn from(columns: Vec<Vector<T>>) -> Matrix<T> {
        if columns.len() > 0 {
            let len_first = columns[0].len();
            for col in columns.iter() {
                assert_eq!(len_first, col.len(), "All columns must be the same length");
            }
        }
        Matrix { columns }
    }

    pub fn identity(rows: usize, columns: usize) -> Matrix<T> {
        assert_eq!(
            rows, columns,
            "Rows needs to equal columns for Identity matrices"
        );

        let columns: Vec<Vector<T>> = (0..columns)
            .map(|i| {
                Vector::new(
                    (0..rows)
                        .map(|j| if i == j { T::one() } else { T::zero() })
                        .collect(),
                )
            })
            .collect();

        Matrix { columns }
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
            .map(|i| Vector::new((0..rows).map(|j| function(i, j)).collect()))
            .collect();
        Matrix { columns }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ncols_and_rows() {
        let m = Matrix {
            columns: vec![
                Vector::new(vec![1, 2, 3]),
                Vector::new(vec![1, 2, 3]),
                Vector::new(vec![1, 2, 3]),
                Vector::new(vec![1, 2, 3]),
            ],
        };
        assert_eq!(m.ncols(), 4);
        assert_eq!(m.nrows(), 3);
    }

    #[test]
    fn test_from() {
        let m = Matrix::from(vec![Vector::new(vec![1])]);
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
        Matrix::from(vec![Vector::new(vec![1]), Vector::new(vec![1, 2])]);
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
