extern crate rand;
extern crate zero_one;
use vector::Vector;
use self::zero_one::{One, Zero};

use self::rand::Rand;

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    columns: Vec<Vector<T>>,
}

impl<T: Zero> Matrix<T> {
    pub fn zero(rows: usize, columns: usize) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..columns)
            .map(|_| Vector::from_vec((0usize..rows).map(|_| T::zero()).collect()))
            .collect();

        Matrix { columns }
    }
}

impl<T: Zero + One> Matrix<T> {
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

        Matrix { columns }
    }
}

impl<T: Rand> Matrix<T> {
    pub fn random(rows: usize, columns: usize) -> Matrix<T> {
        Matrix::from_function(rows, columns, |_, _| rand::random::<T>())
    }
}

impl<T> Matrix<T> {
    pub fn from_vec(columns: Vec<Vector<T>>) -> Matrix<T> {
        if !columns.is_empty() {
            let len_first = columns[0].len();
            for col in columns.iter() {
                assert_eq!(len_first, col.len(), "All columns must be the same length");
            }
        }
        Matrix { columns }
    }

    pub fn from_function(
        rows: usize,
        columns: usize,
        function: fn(usize, usize) -> T,
    ) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..columns)
            .map(|i| Vector::from_vec((0..rows).map(|j| function(i, j)).collect()))
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

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.columns[col][row]
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

        let mut columns: Vec<Vector<&T>> = Vec::with_capacity(col + cols);
        for col in &self.columns[col..col + cols] {
            let rows = &col[row..row + rows];
            let rows = rows.iter().map(|x| &**x).collect::<Vec<&T>>();

            columns.push(Vector::from_vec(rows));
        }

        Matrix { columns }
    }
}

impl<T: Copy> Matrix<T> {
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.columns[col][row]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.columns[col][row] = value;
    }

    pub fn set_segment(&mut self, row: usize, col: usize, segment: Matrix<T>) {
        let rows = segment.nrows();
        let cols = segment.ncols();
        assert!(
            row + rows < self.nrows(),
            "Index out of bounds: too many rows"
        );
        assert!(
            col + cols < self.ncols(),
            "Index out of bounds: too many cols"
        );

        let columns: Vec<Vector<T>> = segment.columns;

        for (i, column) in columns.into_iter().enumerate() {
            for (j, element) in column.into_iter().enumerate() {
                self.columns[col + i][row + j] = *element;
            }
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
            ],
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

    #[test]
    fn test_from_function() {
        let m = Matrix::from_function(10, 10, |_, _| 1);
        // check size
        assert_eq!(10, m.columns.len());
        for i in 0..10 {
            assert_eq!(10, m.columns[i].len());
        }
        let acc: i32 = (0..10)
            .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
            .sum();
        assert_eq!(100, acc);
    }

    #[test]
    fn test_get() {
        let m = Matrix::from_function(10, 10, |x, y| 10 * x + y);
        assert_eq!(&92, m.get(2, 9));
    }

    #[test]
    fn test_set() {
        let mut m = Matrix::zero(10, 10);
        m.set(3, 4, 1);
        assert_eq!(m.columns[4][3], 1);
    }

    #[test]
    fn test_set_segment() {
        let mut m: Matrix<i32> = Matrix::zero(10, 10);
        let t = Matrix::identity(3, 3);
        m.set_segment(3, 3, t);

        assert_eq!(m.columns[3][3], 1);
        assert_eq!(m.columns[4][4], 1);
        assert_eq!(m.columns[5][5], 1);
        let acc: i32 = (0..10)
            .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
            .sum();
        assert_eq!(3, acc);
    }

}
