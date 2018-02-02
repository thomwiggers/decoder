extern crate zero_one;
extern crate rand;
use vector::Vector;
use self::zero_one::{Zero, One};

use self::rand::Rand;


pub struct Matrix<T>{
    columns: Vec<Vector<T>>
}

impl<T: Zero + One + Rand> Matrix<T> {
    pub fn zero(rows: usize, columns: usize) -> Matrix<T> {
        let columns: Vec<Vector<T>> = 
            (0..columns)
            .map(
                |_| {
                    Vector::new(
                        (0usize..rows)
                            .map(|_| T::zero())
                            .collect()
                    )
                })
            .collect();

        Matrix {
            columns
        }
    }

    pub fn identity(rows: usize, columns: usize) -> Matrix<T> {
        assert_eq!(rows, columns, "Rows needs to equal columns for Identity matrices");

        let columns: Vec<Vector<T>> = (0..columns)
            .map(|i| {
                Vector::new(
                    (0..rows)
                        .map(|j| {
                            if i == j {
                                T::one()
                            } else {
                                T::zero()
                            }
                        }).collect()
                )
            }).collect();

        Matrix {
            columns
        }
    }

    pub fn random(rows: usize, columns: usize) -> Matrix<T> {
        let columns: Vec<Vector<T>> = (0..columns)
            .map(|_| {
                Vector::new(
                    (0..rows)
                        .map(|_| {
                            rand::random::<T>()
                        })
                        .collect()
                )
            })
            .collect();

        Matrix {
            columns
        }
    }
}
