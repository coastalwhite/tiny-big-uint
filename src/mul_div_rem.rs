use crate::BigUInt;
use core::ops::{Div, DivAssign, Mul, MulAssign, Rem, RemAssign};

impl<const NUM_WORDS: usize> Mul<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn mul(self, mut rhs: Self) -> Self::Output {
        // TLDR: Multiplying is repeated addition
        // I know :(
        
        let mut big_int = BigUInt::MIN;

        loop {
            if rhs.is_zero() {
                break big_int;
            }
            rhs = rhs.decrease();

            big_int = big_int + self.clone();
        }
    }
}
impl<const NUM_WORDS: usize> Div<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        // TLDR: Dividing is a counter of repeated subtraction
        // Again, I know :(
        
        let mut big_int = BigUInt::MIN;

        loop {
            if rhs > self {
                break big_int;
            }

            self = self - rhs.clone();
            big_int = big_int.increase();
        }
    }
}
impl<const NUM_WORDS: usize> Rem<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn rem(mut self, rhs: Self) -> Self::Output {
        // TLDR: Modulo is repeated subtraction
        // Once again, I know :(
        
        loop {
            if rhs > self {
                break self;
            }
            self = self - rhs.clone();
        }
    }
}

impl<const NUM_WORDS: usize> MulAssign<Self> for BigUInt<NUM_WORDS> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}
impl<const NUM_WORDS: usize> DivAssign<Self> for BigUInt<NUM_WORDS> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}
impl<const NUM_WORDS: usize> RemAssign<Self> for BigUInt<NUM_WORDS> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs;
    }
}

#[test]
fn mul() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN * BigUInt::MIN, BigUInt::<128>::MIN);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(21u32) * <BigUInt<128>>::from(2u32),
        <BigUInt<128>>::from(42u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(250u32) * <BigUInt<128>>::from(40u32),
        <BigUInt<128>>::from(10000u32)
    );

    // Small Loop check
    for x in 250..260u32 {
        for y in 250..260u32 {
            assert_eq!(
                <BigUInt<16>>::from(x) * <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x * y)
            );
        }
    }

    // Bigger Loop check
    for x in 10000..10010u32 {
        for y in 10000..10010u32 {
            assert_eq!(
                <BigUInt<16>>::from(x) * <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x * y)
            );
        }
    }
}
#[test]
fn div() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN / BigUInt::MAX, BigUInt::<128>::MIN);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(21u32) / <BigUInt<128>>::from(3u32),
        <BigUInt<128>>::from(7u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(10000u32) / <BigUInt<128>>::from(40u32),
        <BigUInt<128>>::from(250u32)
    );

    // Small Loop check
    for x in 250..260u32 {
        for y in 250..260u32 {
            assert_eq!(
                <BigUInt<16>>::from(x * y) / <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x)
            );
        }
    }

    // Bigger Loop check
    for x in 10000..10010u32 {
        for y in 10000..10010u32 {
            assert_eq!(
                <BigUInt<16>>::from(x * y) / <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x)
            );
        }
    }
}

#[test]
fn rem() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN % BigUInt::MAX, BigUInt::<128>::MIN);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(21u32) % <BigUInt<128>>::from(5u32),
        <BigUInt<128>>::from(1u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(10000u32) % <BigUInt<128>>::from(40u32),
        <BigUInt<128>>::MIN
    );

    // Small Loop check
    for x in 250..260u32 {
        for y in 250..260u32 {
            assert_eq!(
                <BigUInt<16>>::from(x) % <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x % y)
            );
        }
    }

    // Bigger Loop check
    for x in 10000..10010u32 {
        for y in 10000..10010u32 {
            assert_eq!(
                <BigUInt<16>>::from(x) % <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x % y)
            );
        }
    }
}
