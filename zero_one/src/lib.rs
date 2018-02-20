pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_zero {
    ($t: ty, $v: expr) => {
        impl Zero for $t {
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
