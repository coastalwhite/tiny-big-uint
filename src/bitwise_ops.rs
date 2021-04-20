use crate::BigUInt;
use core::ops::{BitAnd, BitOr, BitXor, BitOrAssign, BitAndAssign, BitXorAssign};

impl<const NUM_WORDS: usize> BitAnd<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut internal = [0; NUM_WORDS];

        for i in 0..NUM_WORDS {
            internal[i] = self.internal[i] & rhs.internal[i];
        }

        BigUInt { internal }
    }
}
impl<const NUM_WORDS: usize> BitOr<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut internal = [0; NUM_WORDS];

        for i in 0..NUM_WORDS {
            internal[i] = self.internal[i] | rhs.internal[i];
        }

        BigUInt { internal }
    }
}
impl<const NUM_WORDS: usize> BitXor<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut internal = [0; NUM_WORDS];

        for i in 0..NUM_WORDS {
            internal[i] = self.internal[i] ^ rhs.internal[i];
        }

        BigUInt { internal }
    }
}

impl<const NUM_WORDS: usize> BitOrAssign<Self> for BigUInt<NUM_WORDS> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone() | rhs;
    }
}
impl<const NUM_WORDS: usize> BitAndAssign<Self> for BigUInt<NUM_WORDS> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone() & rhs;
    }
}
impl<const NUM_WORDS: usize> BitXorAssign<Self> for BigUInt<NUM_WORDS> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone() ^ rhs;
    }
}

#[test]
fn xor() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN ^ BigUInt::MIN, BigUInt::<128>::MIN);
    // Check it stays MAX
    assert_eq!(BigUInt::MIN ^ BigUInt::MAX, BigUInt::<128>::MAX);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(42u32) ^ <BigUInt<128>>::from(2u32),
        <BigUInt<128>>::from(40u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(260u32) ^ <BigUInt<128>>::from(20u32),
        <BigUInt<128>>::from(272u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        for y in 200..300u32 {
            assert_eq!(
                <BigUInt<128>>::from(x) ^ <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x ^ y)
            );
        }
    }

    // Bigger Loop check
    for x in 65500..65600u32 {
        for y in 65500..65600u32 {
            assert_eq!(
                <BigUInt<128>>::from(x) ^ <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x ^ y)
            );
        }
    }
}
#[test]
fn or() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN | BigUInt::MIN, BigUInt::<128>::MIN);
    // Check it stays MAX
    assert_eq!(BigUInt::MIN | BigUInt::MAX, BigUInt::<128>::MAX);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(42u32) | <BigUInt<128>>::from(1u32),
        <BigUInt<128>>::from(43u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(260u32) | <BigUInt<128>>::from(20u32),
        <BigUInt<128>>::from(276u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        for y in 200..300u32 {
            assert_eq!(
                <BigUInt<128>>::from(x) | <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x | y)
            );
        }
    }

    // Bigger Loop check
    for x in 65500..65600u32 {
        for y in 65500..65600u32 {
            assert_eq!(
                <BigUInt<128>>::from(x) | <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x | y)
            );
        }
    }
}
#[test]
fn and() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN & BigUInt::MIN, BigUInt::<128>::MIN);
    // Check it stays 0
    assert_eq!(BigUInt::MIN & BigUInt::MAX, BigUInt::<128>::MIN);
    // Check it stays MAX
    assert_eq!(BigUInt::MAX & BigUInt::MAX, BigUInt::<128>::MAX);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(42u32) & <BigUInt<128>>::from(1u32),
        <BigUInt<128>>::from(0u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(260u32) & <BigUInt<128>>::from(20u32),
        <BigUInt<128>>::from(4u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        for y in 200..300u32 {
            assert_eq!(
                <BigUInt<128>>::from(x) & <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x & y)
            );
        }
    }

    // Bigger Loop check
    for x in 65500..65600u32 {
        for y in 65500..65600u32 {
            assert_eq!(
                <BigUInt<128>>::from(x) & <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x & y)
            );
        }
    }
}
