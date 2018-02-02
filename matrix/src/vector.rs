use std::ops::{Add, Sub, Mul};
use std::iter::Sum;
use std::clone::Clone;

#[derive(Debug, PartialEq)]
pub struct Vector<T> {
    elements: Vec<T>
}

impl<T> Vector<T> {
    pub fn new(elements: Vec<T>) -> Vector<T> {
        Vector {
            elements
        }
    }
}

impl<'a, 'b, T: Add<Output=T> + Clone> Add<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    #[inline]
    fn add(self, other: &'b Vector<T>) -> Vector<T> {
        assert_eq!(self.elements.len(), other.elements.len(), 
                   "Vectors should be of equal length");

        Vector { 
            elements: self.elements.clone()
                .into_iter()
                .zip(other.elements.clone())
                .map(|(x, y)| x + y)
                .collect()
        }
    }
}

impl<'a, 'b, T: Sub<Output=T> + Clone> Sub<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    #[inline]
    fn sub(self, other: &'b Vector<T>) -> Vector<T> {
        assert_eq!(self.elements.len(), other.elements.len());

        Vector { 
            elements: self.elements.clone()
                .into_iter()
                .zip(other.elements.clone())
                .map(|(x, y)| x - y)
                .collect()
        }
    }
}

impl<'a, 'b, T: Mul<Output=T> + Sum<T> + Clone> Mul<&'b Vector<T>> for &'a Vector<T> {
    type Output = T;

    #[inline]
    fn mul(self, other: &'b Vector<T>) -> T {
        assert_eq!(self.elements.len(), other.elements.len());

        self.elements.clone()
            .into_iter()
            .zip(other.elements.clone())
            .map(|(x, y)| x * y)
            .sum()
    }
}
        

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

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

    fn get_two_vectors(size: u32) -> (Vector<u32>, Vector<u32>) {
        let mut v1 = Vec::with_capacity(10000);
        let mut v2 = Vec::with_capacity(10000);
        for i in 0 .. size {
            v1.push(i);
            v2.push(i);
        }
        let v1 = Vector::new(v1);
        let v2 = Vector::new(v2);
        (v1, v2)
    }

    #[bench]
    fn bench_add_vector(b: &mut Bencher) {
        let (v1, v2) = get_two_vectors(10000);

        b.iter(|| {
            &v1 + &v2
        })
    }

    #[bench]
    fn bench_sub_vector(b: &mut Bencher) {
        let (v1, v2) = get_two_vectors(10000);

        b.iter(|| {
            &v1 - &v2
        })
    }
}
