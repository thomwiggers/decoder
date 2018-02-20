#![feature(type_ascription)]
#![feature(test)]
extern crate test;

mod vector;
mod matrix;

pub use vector::Vector;
pub use matrix::Matrix;
