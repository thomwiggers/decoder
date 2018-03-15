#![feature(test)]

extern crate matrix;
extern crate test;

#[cfg(test)]
mod tests {
    use matrix::Vector;
    use test::Bencher;

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
