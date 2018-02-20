use std::ops::{Add, Index, Mul, Sub};
use std::iter::Sum;
use std::clone::Clone;

#[derive(Debug, PartialEq)]
pub struct Vector<T> {
    elements: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(elements: Vec<T>) -> Vector<T> {
        Vector { elements }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.elements[idx]
    }
}

impl<'a, 'b, T: Add<Output = T> + Clone> Add<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    #[inline]
    fn add(self, other: &'b Vector<T>) -> Vector<T> {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        Vector {
            elements: self.elements
                .iter()
                .zip(other.elements.iter())
                .map(|(x, y)| x.clone() + y.clone())
                .collect(),
        }
    }
}

impl<T: Add<Output = T>> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    #[inline]
    fn add(self, other: Vector<T>) -> Vector<T> {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        Vector {
            elements: self.elements
                .into_iter()
                .zip(other.elements)
                .map(|(x, y)| x + y)
                .collect(),
        }
    }
}

impl<T: Sub<Output = T>> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    #[inline]
    fn sub(self, other: Vector<T>) -> Vector<T> {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        Vector {
            elements: self.elements
                .into_iter()
                .zip(other.elements)
                .map(|(x, y)| x - y)
                .collect(),
        }
    }
}

impl<'a, 'b, T: Sub<Output = T> + Clone> Sub<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    #[inline]
    fn sub(self, other: &'b Vector<T>) -> Vector<T> {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        Vector {
            elements: self.elements
                .iter()
                .zip(other.elements.iter())
                .map(|(x, y)| x.clone() - y.clone())
                .collect(),
        }
    }
}

impl<'a, 'b, T: Mul<Output = T> + Sum<T> + Clone> Mul<&'b Vector<T>> for &'a Vector<T> {
    type Output = T;

    #[inline]
    fn mul(self, other: &'b Vector<T>) -> T {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(x, y)| x.clone() * y.clone())
            .sum()
    }
}

impl<T: Mul<Output = T> + Sum<T>> Mul<Vector<T>> for Vector<T> {
    type Output = T;

    #[inline]
    fn mul(self, other: Vector<T>) -> T {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        self.elements
            .into_iter()
            .zip(other.elements)
            .map(|(x, y)| x * y)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn len() {
        let v1: Vector<i32> = Vector::new(vec![1, 2, 3]);
        assert_eq!(v1.len(), 3);
        let v2: Vector<i32> = Vector::new(vec![1, 2]);
        assert_eq!(v2.len(), 2);
    }

    #[test]
    fn add_vectors() {
        let v1: Vector<i32> = Vector::new(vec![0, 1, 2]);
        let v2: Vector<i32> = Vector::new(vec![0, 1, 2]);
        let v3 = &v1 + &v2;
        assert_eq!(v3.elements, [0, 2, 4]);
    }

    #[test]
    fn sub_vectors() {
        let v1: Vector<i32> = Vector::new(vec![0, 1, 2]);
        let v2: Vector<i32> = Vector::new(vec![0, 1, 2]);
        let v3 = &v1 - &v2;
        assert_eq!(v3.elements, [0, 0, 0]);
    }

    #[test]
    fn dot_product() {
        let v1: Vector<i32> = Vector::new(vec![1, 3, -5]);
        let v2: Vector<i32> = Vector::new(vec![4, -2, -1]);
        assert_eq!(&v1 * &v2, 3i32);
        assert_eq!(v1 * v2, 3i32);
    }

    #[test]
    #[should_panic]
    fn add_diff_sized() {
        &Vector::new(vec![0]) + &Vector::new(vec![0, 1]);
    }

    #[test]
    #[should_panic]
    fn sub_diff_sized() {
        &Vector::new(vec![0]) - &Vector::new(vec![0, 1]);
    }

    #[test]
    fn test_get_index() {
        let vec = Vector::new(vec![1, 2, 3]);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    #[should_panic]
    fn get_index_out_of_bounds() {
        Vector::new(vec![1])[100];
    }

    fn get_two_vectors(size: usize) -> (Vector<usize>, Vector<usize>) {
        let mut v1 = Vec::with_capacity(size);
        let mut v2 = Vec::with_capacity(size);
        for i in 0..size {
            v1.push(i);
            v2.push(i);
        }
        let v1 = Vector::new(v1);
        let v2 = Vector::new(v2);
        (v1, v2)
    }

    #[bench]
    fn bench_add_vector_borrowed(b: &mut Bencher) {
        b.iter(|| {
            let (v1, v2) = get_two_vectors(100);
            &v1 + &v2
        })
    }

    #[bench]
    fn bench_add_vector_move(b: &mut Bencher) {
        b.iter(|| {
            let (v1, v2) = get_two_vectors(100);
            v1 + v2
        })
    }

    #[bench]
    fn bench_sub_vector_borrowed(b: &mut Bencher) {
        b.iter(|| {
            let (v1, v2) = get_two_vectors(100);
            &v1 - &v2
        })
    }

    #[bench]
    fn bench_sub_vector_move(b: &mut Bencher) {
        b.iter(|| {
            let (v1, v2) = get_two_vectors(100);
            v1 - v2
        })
    }

    #[bench]
    fn bench_mul_vector_borrowed(b: &mut Bencher) {
        b.iter(|| {
            let (v1, v2) = get_two_vectors(100);
            &v1 * &v2
        })
    }

    #[bench]
    fn bench_mul_vector_move(b: &mut Bencher) {
        b.iter(|| {
            let (v1, v2) = get_two_vectors(100);
            v1 * v2
        })
    }
}
