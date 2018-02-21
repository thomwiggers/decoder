use std::ops;
use std::iter::Sum;
use std::clone::Clone;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Vector<T> {
    elements: Vec<Rc<T>>,
}

impl<T> Vector<T> {
    pub fn from_vec(elements: Vec<T>) -> Vector<T> {
        let elements = elements.into_iter().map(Rc::new).collect();
        Vector { elements: elements }
    }

    pub fn from(elements: Box<[T]>) -> Vector<T> {
        Vector::from_vec(elements.into_vec())
    }

    pub fn repeat(n: usize, element: T) -> Vector<T> {
        let element = Rc::new(element);
        Vector {
            elements: ((0..n).map(|_| element.clone()).collect()),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Clone for Vector<T> {
    fn clone(&self) -> Self {
        Vector {
            elements: self.elements.iter().map(|e| e.clone()).collect(),
        }
    }
}

impl<T> ops::Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &*self.elements[idx]
    }
}

impl<'a, T> ops::Index<ops::Range<usize>> for Vector<T> {
    type Output = [Rc<T>];
    fn index(&self, idxs: ops::Range<usize>) -> &Self::Output {
        &self.elements[idxs]
    }
}

impl<T: Clone> ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, idx: usize) -> &mut T {
        Rc::make_mut(&mut self.elements[idx])
    }
}

macro_rules! binary_operator {
    ($type:ident, $funcname:ident, $operator:tt) => {
        impl<'a, T: ops::$type<Output = T>> ops::$type<&'a Vector<T>> for &'a Vector<T>
            where &'a T: ops::$type<Output = T> 
        {
            type Output = Vector<T>;

            #[inline]
            fn $funcname(self, other: &'a Vector<T>) -> Vector<T> {
                assert_eq!(
                    self.elements.len(),
                    other.elements.len(),
                    "Vectors should be of equal length"
                );

                let elements: Vec<Rc<T>> = self.elements
                    .iter()
                    .zip(other.elements.iter())
                    .map(|(x, y)| {
                        let x: &T = &*x;
                        let y: &T = &*y;
                        Rc::new(x $operator y)
                    })
                    .collect();

                Vector {
                    elements: elements
                }
            }
        }

        impl<T: ops::$type<Output = T> + Copy> ops::$type<Vector<T>> for Vector<T> {
            type Output = Vector<T>;

            #[inline]
            fn $funcname(self, other: Vector<T>) -> Vector<T> {
                assert_eq!(
                    self.elements.len(),
                    other.elements.len(),
                    "Vectors should be of equal length"
                );

                Vector {
                    elements: self.elements
                        .into_iter()
                        .zip(other.elements)
                        .map(|(x, y)| Rc::new(*x $operator *y))
                        .collect()
                }
            }
        }
    }
}

binary_operator!(Add, add, +);
binary_operator!(Sub, sub, -);


impl<'a, T: ops::Mul<Output = T> + Sum<T>> ops::Mul<&'a Vector<T>> for &'a Vector<T>
    where &'a T: ops::Mul<Output = T>
{
    type Output = T;

    #[inline]
    fn mul(self, other: &'a Vector<T>) -> T {
        assert_eq!(
            self.elements.len(),
            other.elements.len(),
            "Vectors should be of equal length"
        );

        self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(x, y)| {
                let x: &T = &*x;
                let y: &T = &*y;
                x * y
            })
            .sum()
    }
}

impl<T: ops::Mul<Output = T> + Sum<T> + Copy> ops::Mul<Vector<T>> for Vector<T> 
{
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
            .map(|(x, y)| *x * *y)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn len() {
        let v1: Vector<i32> = Vector::from_vec(vec![1, 2, 3]);
        assert_eq!(v1.len(), 3);
        let v2: Vector<i32> = Vector::from_vec(vec![1, 2]);
        assert_eq!(v2.len(), 2);
    }

    #[test]
    fn add_vectors() {
        let v1: Vector<i32> = Vector::from_vec(vec![0, 1, 2]);
        let v2: Vector<i32> = Vector::from_vec(vec![0, 1, 2]);
        let v3 = &v1 + &v2;
        let els: Vec<i32> = v3.elements.iter().map(|x| **x).collect();
        assert_eq!(els, [0, 2, 4]);
        let v3 = v1 + v2;
        let els: Vec<i32> = v3.elements.iter().map(|x| **x).collect();
        assert_eq!(els, [0, 2, 4]);
    }

    #[test]
    fn sub_vectors() {
        let v1: Vector<i32> = Vector::from_vec(vec![0, 1, 2]);
        let v2: Vector<i32> = Vector::from_vec(vec![0, 1, 2]);
        let v3 = &v1 - &v2;
        let els: Vec<i32> = v3.elements.iter().map(|x| **x).collect();
        assert_eq!(els, [0, 0, 0]);
    }

    #[test]
    fn dot_product() {
        let v1: Vector<i32> = Vector::from_vec(vec![1, 3, -5]);
        let v2: Vector<i32> = Vector::from_vec(vec![4, -2, -1]);
        assert_eq!(&v1 * &v2, 3i32);
        assert_eq!(v1 * v2, 3i32);
    }

    #[test]
    #[should_panic]
    fn add_diff_sized() {
        &Vector::from_vec(vec![0]) + &Vector::from_vec(vec![0, 1]);
    }

    #[test]
    #[should_panic]
    fn sub_diff_sized() {
        &Vector::from_vec(vec![0]) - &Vector::from_vec(vec![0, 1]);
    }

    #[test]
    fn test_get_index() {
        let vec = Vector::from_vec(vec![1, 2, 3]);
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    #[should_panic]
    fn get_index_out_of_bounds() {
        &Vector::from_vec(vec![1])[100];
    }

    #[test]
    fn test_assign_works() {
        let mut vec = Vector::from_vec(vec![1, 2, 3]);
        vec[1] = 4;
        assert_eq!(vec[1], 4);
    }

    #[test]
    fn test_repeat() {
        let vec = Vector::repeat(3, 0);
        let els: Vec<i32> = vec.elements.iter().map(|x| **x).collect();
        assert_eq!(els, [0, 0, 0]);
        assert_eq!(Rc::strong_count(&vec.elements[1]), 3);
        assert!(Rc::ptr_eq(&vec.elements[0], &vec.elements[1]));
        assert!(Rc::ptr_eq(&vec.elements[1], &vec.elements[2]));
    }

    #[test]
    fn test_assign_respects_references() {
        let mut vec = Vector::repeat(3, 0);
        vec[1] = 4;
        let els: Vec<i32> = vec.elements.iter().map(|x| **x).collect();
        assert_eq!(els, [0, 4, 0]);
        assert!(Rc::ptr_eq(&vec.elements[0], &vec.elements[2]));
        assert!(!Rc::ptr_eq(&vec.elements[1], &vec.elements[2]));
    }

    fn get_two_vectors(size: usize) -> (Vector<usize>, Vector<usize>) {
        let mut v1 = Vec::with_capacity(size);
        let mut v2 = Vec::with_capacity(size);
        for i in 0..size {
            v1.push(i);
            v2.push(i);
        }
        let v1 = Vector::from_vec(v1);
        let v2 = Vector::from_vec(v2);
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
