//! This module will allow to work with bit vectors

extern crate zero_one;

use std::ops::{Add, AddAssign};
use std::ops::{BitXor, BitXorAssign};
use std::ops::{Sub, SubAssign};

use zero_one::{Zero, One};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Bit<T> {
    value: T
}

impl<T: Zero + One> Bit<T> {
    pub fn zero() -> Bit<T> {
        Bit {
            value: T::zero()
        }
    }

    pub fn one() -> Bit<T> {
        Bit {
            value: T::one()
        }
    }
}

impl<T> Bit<T> {
    pub fn from(value: T) -> Bit<T> {
        Bit {
            value
        }
    }
}


impl<T: BitXor<Output=T>> Add for Bit<T> {
    type Output = Bit<T>;
    #[inline]
    fn add(self, other: Bit<T>) -> Bit<T> {
        self ^ other
    }
}

impl<T: BitXorAssign<T>> BitXorAssign for Bit<T> {
    fn bitxor_assign(&mut self, other: Bit<T>) {
        self.value ^= other.value
    }
}

impl<T: BitXorAssign<T>> AddAssign for Bit<T> {
    fn add_assign(&mut self, other: Bit<T>) {
        *self ^= other
    }
}

impl<T: BitXorAssign<T>> SubAssign for Bit<T> {
    fn sub_assign(&mut self, other: Bit<T>) {
        *self ^= other
    }
}

impl<T: BitXor<Output=T>> BitXor for Bit<T> {
    type Output = Bit<T>;
    #[inline]
    fn bitxor(self, other: Bit<T>) -> Bit<T> {
        Bit { 
            value: self.value ^ other.value 
        }
    }
}

impl<T: BitXor<Output=T>> Sub for Bit<T> {
    type Output = Bit<T>;
    #[inline]
    fn sub(self, other: Bit<T>) -> Bit<T> {
        self ^ other
    }
}


macro_rules! tests_for_type {
    ($type: ty, $name: ident, $zero: expr, $one: expr) => {

        #[cfg(test)]
        mod $name {
            use super::Bit;

            const ZERO: $type = $zero;
            const ONE: $type = $one;

            #[test]
            fn zero() {
                let bit: Bit<$type> = Bit::zero();
                assert_eq!(bit.value, ZERO);
            }

            #[test]
            fn from() {
                let bit: Bit<$type> = Bit::from(ZERO);
                assert_eq!(bit.value, ZERO);
            }

            #[test]
            fn one() {
                let bit: Bit<$type> = Bit::one();
                assert_eq!(bit.value, ONE);
            }

            #[test]
            fn zero_plus_zero_eq_zero() {
                let bit: Bit<$type> = Bit::zero();
                assert_eq!(bit + bit, Bit::zero());
            }

            #[test]
            fn zero_sub_zero_eq_zero() {
                let bit: Bit<$type> = Bit::zero();
                assert_eq!(bit - bit, Bit::zero());
            }

            #[test]
            fn add_assign() {
                let mut bit: Bit<$type> = Bit::zero();
                bit += Bit::one();
                assert_eq!(bit, Bit::one());
                bit += Bit::one();
                assert_eq!(bit, Bit::zero());
            }
            
            #[test]
            fn sub_assign() {
                let mut bit: Bit<$type> = Bit::zero();
                bit -= Bit::one();
                assert_eq!(bit, Bit::one());
                bit -= Bit::one();
                assert_eq!(bit, Bit::zero());
            }
            #[test]
            fn zero_plus_one_eq_one() {
                let zero: Bit<$type> = Bit::zero();
                let one: Bit<$type>= Bit::one();
                assert_eq!(zero + one, one);
                assert_eq!(one + zero, one);
            }
            
            #[test]
            fn one_plus_one_eq_zero() {
                let zero: Bit<$type> = Bit::zero();
                let one: Bit<$type>= Bit::one();
                assert_eq!(one + one, zero);
            }
        }
    }
}

tests_for_type!(bool, tests_bool, false, true);
tests_for_type!(i32, tests_i32, 0i32, 1i32);
tests_for_type!(u32, tests_u32, 0u32, 1u32);
tests_for_type!(i64, tests_i64, 0i64, 1i64);
tests_for_type!(u64, tests_u64, 0u64, 1u64);
