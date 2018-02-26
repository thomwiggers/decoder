extern crate rand;
extern crate zero_one;
use vector::Vector;
use self::zero_one::{One, Zero};
use std::ops;

use self::rand::Rand;

#[derive(Clone, Debug, PartialEq)]
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
    pub fn identity(size: usize) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..size)
            .map(|i| {
                Vector::from_vec(
                    (0..size)
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
            for col in &columns {
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

    pub fn get_segment(&self, row: usize, col: usize, rows: usize, cols: usize) -> Matrix<T> {
        assert!(
            row + rows < self.nrows(),
            "Index out of bounds: too many rows"
        );
        assert!(
            col + cols < self.ncols(),
            "Index out of bounds: too many cols"
        );

        let mut columns: Vec<Vector<T>> = Vec::with_capacity(col + cols);
        for col in &self.columns[col..col + cols] {
            let rows = &col[row..row + rows];
            let rows = rows.to_vec();

            columns.push(Vector::from_rc_vec(rows));
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

macro_rules! pointwise_operator {
    ($type:ident, $funcname:ident, $operator:tt) => {
        impl<T> ops::$type for Matrix<T>
            where Vector<T>: ops::$type<Output=Vector<T>>
        {
            type Output = Matrix<T>;

            fn $funcname(self, other: Matrix<T>) -> Self::Output {
                debug_assert_eq!(self.ncols(), other.nrows(),
                            "they should have the same number of columns");
                let columns: Vec<Vector<T>> = self.columns
                    .into_iter()
                    .zip(other.columns)
                    .map(|(a, b)| a $operator b)
                    .collect::<Vec<Vector<T>>>();

                Matrix { columns }
            }
        }

        impl<'a, T> ops::$type for &'a Matrix<T>
            where Vector<T>: ops::$type<Output=Vector<T>>
        {
            type Output = Matrix<T>;

            fn $funcname(self, other: &Matrix<T>) -> Self::Output {
                let columns: Vec<Vector<T>> = self.columns
                    .iter()
                    .zip(other.columns.iter())
                    .map(|(a, b)| a.clone() $operator b.clone())
                    .collect::<Vec<Vector<T>>>();

                Matrix { columns }
            }
        }

    }
}

pointwise_operator!(Add, add, +);
pointwise_operator!(Sub, sub, -);

impl<'a, T> ops::Mul<&'a Matrix<T>> for &'a Vector<T>
where
    &'a Vector<T>: ops::Mul<Output = T>,
{
    type Output = Vector<T>;

    fn mul(self, other: &'a Matrix<T>) -> Self::Output {
        debug_assert_eq!(
            self.len(),
            other.nrows(),
            "The number of columns should match the number of rows"
        );
        let result: Vec<T> = other.columns.iter().map(|c| self * c).collect();

        Vector::from_vec(result)
    }
}

impl<'a, T> ops::Mul<Matrix<T>> for Vector<T>
where
    Vector<T>: ops::Mul<Output = T>, /*
    where &'a Vector<T>: ops::Mul<&'a Matrix<T>>,
          &'a Vector<T>: ops::Mul<Output=T>,
          Vector<T>: 'a,
*/
{
    type Output = Vector<T>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        debug_assert_eq!(
            self.len(),
            other.nrows(),
            "The length of vector should match the number of matrix rows"
        );
        let result: Vec<T> = other
            .columns
            .into_iter()
            .map(|c| self.clone() * c)
            .collect();

        Vector::from_vec(result)
        //&self * &other;
    }
}

impl<T> ops::Mul<Matrix<T>> for Matrix<T>
where
    Matrix<T>: Clone,
    Vector<T>: ops::Mul<Matrix<T>, Output = Vector<T>>,
{
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Self::Output {
        debug_assert_eq!(
            self.ncols(),
            other.nrows(),
            "The number of columns should match the number of rows"
        );
        let columns: Vec<Vector<T>> = self.columns
            .into_iter()
            .map(|c| c * other.clone())
            .collect();
        Matrix { columns }
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
        let m: Matrix<i32> = Matrix::identity(10);
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
    fn test_get_segment() {
        let m: Matrix<i32> = Matrix::identity(10);
        let m3 = m.get_segment(0, 0, 3, 3);
        assert_eq!(m3.nrows(), 3);
        assert_eq!(m3.ncols(), 3);
        let acc: i32 = (0..3)
            .map(|i| (0..3).map(|j| m.columns[i][j]).sum(): i32)
            .sum();
        assert_eq!(3, acc);
    }

    #[test]
    fn test_set_segment() {
        let mut m: Matrix<i32> = Matrix::zero(10, 10);
        let t = Matrix::identity(3);
        m.set_segment(3, 3, t);

        assert_eq!(m.columns[3][3], 1);
        assert_eq!(m.columns[4][4], 1);
        assert_eq!(m.columns[5][5], 1);
        let acc: i32 = (0..10)
            .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
            .sum();
        assert_eq!(3, acc);
    }

    macro_rules! get_test_with_accumulator {
        ($operator: tt) => {
            fn _test(m1: Matrix<i32>, m2: Matrix<i32>, expected_sum: i32) -> Matrix<i32> {
                let m = &m1 $operator &m2;
                let acc: i32 = (0..10)
                    .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
                    .sum();
                assert_eq!(expected_sum, acc);

                let m = m1 $operator m2;
                let acc: i32 = (0..10)
                    .map(|i| (0..10).map(|j| m.columns[i][j]).sum(): i32)
                    .sum();
                assert_eq!(expected_sum, acc);
                m
            }
        }
    }

    #[test]
    fn test_addition() {
        get_test_with_accumulator!(+);

        let m1: Matrix<i32> = Matrix::zero(10, 10);
        let m2: Matrix<i32> = Matrix::identity(10);
        _test(m1, m2, 10);

        let m1 = Matrix::identity(10);
        let m2 = Matrix::identity(10);
        let m = _test(m1, m2, 20);
        assert_eq!(m.columns[0][0], 2);
    }

    #[test]
    #[should_panic]
    fn test_addition_different_col_size() {
        let m1: Matrix<i32> = Matrix::zero(1, 3);
        let m2: Matrix<i32> = Matrix::zero(1, 4);
        m1 + m2;
    }

    #[test]
    #[should_panic]
    fn test_addition_different_row_size() {
        let m1: Matrix<i32> = Matrix::zero(2, 3);
        let m2: Matrix<i32> = Matrix::zero(1, 3);
        m1 + m2;
    }

    #[test]
    fn test_subtraction() {
        get_test_with_accumulator!(-);
        let m1: Matrix<i32> = Matrix::zero(10, 10);
        let m2: Matrix<i32> = Matrix::identity(10);
        let m = _test(m1, m2, -10);
        assert_eq!(m.columns[0][0], -1);

        let m1 = Matrix::identity(10);
        let m2 = Matrix::identity(10);
        let m = _test(m1, m2, 0);
        assert_eq!(m.columns[0][0], 0);
    }

    #[test]
    fn test_vector_mul_reference() {
        let v: Vector<i32> = Vector::repeat(10, 1);
        let m: Matrix<i32> = Matrix::identity(10);

        let result: Vector<i32> = &v * &m;

        assert_eq!(result.len(), 10);
        assert_eq!(result[0], 1, "Position 0 should be 1");
        for i in 1..10 {
            assert_eq!(result[i], 1, "Position {} should be 0", i);
        }

        let v = Vector::repeat(10, 1);
        let mut m = Matrix::zero(10, 10);
        m.columns[0][1] = 1;
        let result = &v * &m;
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], 1, "Position 0 should be 1");
        for i in 1..10 {
            assert_eq!(result[i], 0, "Position {} should be 0", i);
        }
    }

    #[test]
    fn test_vector_mul_no_reference() {
        let v: Vector<i32> = Vector::repeat(10, 1);
        let m: Matrix<i32> = Matrix::identity(10);

        let result: Vector<i32> = &v * &m;

        assert_eq!(result.len(), 10);
        assert_eq!(result[0], 1, "Position 0 should be 1");
        for i in 1..10 {
            assert_eq!(result[i], 1, "Position {} should be 0", i);
        }

        let v = Vector::repeat(10, 1);
        let mut m = Matrix::zero(10, 10);
        m.columns[0][1] = 1;
        let result = &v * &m;
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], 1, "Position 0 should be 1");
        for i in 1..10 {
            assert_eq!(result[i], 0, "Position {} should be 0", i);
        }
    }

    #[test]
    fn test_matrix_mul_no_reference() {
        let m1: Matrix<i32> = Matrix::random(10, 10);
        let m2: Matrix<i32> = Matrix::identity(10);
        let m = m1.clone() * m2;
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(m.columns[i][j], m1.columns[i][j]);
            }
        }
    }
}
