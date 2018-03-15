//! # zero_one
//! This crate defines functions to get at the zero and one (unit) elements of
//! certain types. This allows to instantiate generics over these types with zero
//! or one values.
//!
//! ```
//! use zero_one::One;
//! struct MyStruct<T>(T);
//! impl<T: One> One for MyStruct<T> {
//!     fn one() -> MyStruct<T> {
//!         MyStruct(T::one())
//!     }
//! }
//! ```

pub trait Zero {
    /// Get the zero element
    ///
    /// The zero element is useful to implement group laws,
    /// i.e. `zero + a = a`.
    ///
    /// # Examples
    /// ```
    /// use zero_one::Zero;
    /// assert_eq!(u32::zero(), 0u32);
    /// assert_eq!(i32::zero(), 0i32);
    /// assert_eq!(u64::zero(), 0u64);
    /// assert_eq!(i64::zero(), 0i64);
    /// assert_eq!(bool::zero(), false);
    /// ```
    fn zero() -> Self;
}

pub trait One {
    /// Get the one element
    ///
    /// The one element is useful to implement group laws,
    /// i.e. `one * a = a`.
    ///
    /// # Examples
    /// ```
    /// use zero_one::One;
    /// assert_eq!(u32::one(), 1u32);
    /// assert_eq!(i32::one(), 1i32);
    /// assert_eq!(u64::one(), 1u64);
    /// assert_eq!(i64::one(), 1i64);
    /// assert_eq!(bool::one(), true);
    /// ```
    fn one() -> Self;
}

macro_rules! impl_zero {
    ($t: ty, $v: expr) => {
        impl Zero for $t {
            /// Returns the one element for T, which conforms to
            /// ```no_compile
            /// T::zero() + a == a;
            /// ```
            #[inline]
            fn zero() -> $t {
                $v
            }
        }
    }
}

impl_zero!(u32, 0u32);
impl_zero!(i32, 0i32);
impl_zero!(u64, 0u64);
impl_zero!(i64, 0i64);
impl_zero!(bool, false);

macro_rules! impl_one {
    ($t: ty, $v: expr) => {
        impl One for $t {
            /// Returns the one element for T, which conforms to
            /// ```no_compile
            /// T::one() * a == a;
            /// ```
            #[inline]
            fn one() -> $t {
                $v
            }
        }
    }
}

impl_one!(u32, 1u32);
impl_one!(i32, 1i32);
impl_one!(u64, 1u64);
impl_one!(i64, 1i64);
impl_one!(bool, true);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(u32::zero(), 0u32);
        assert_eq!(i32::zero(), 0i32);
        assert_eq!(u64::zero(), 0u64);
        assert_eq!(i64::zero(), 0i64);
        assert_eq!(bool::zero(), false);
    }

    #[test]
    fn one() {
        assert_eq!(u32::one(), 1u32);
        assert_eq!(i32::one(), 1i32);
        assert_eq!(u64::one(), 1u64);
        assert_eq!(i64::one(), 1i64);
        assert_eq!(bool::one(), true);
    }
}
