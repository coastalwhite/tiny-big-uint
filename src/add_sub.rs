use crate::{BigUInt, Word};
use core::ops::{Add, AddAssign, Sub, SubAssign};

impl<const NUM_WORDS: usize> Add<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut carry = false;
        let mut big_int = BigUInt::MIN;

        // Go over all the words and declare the sum of each word.
        for i in 0..NUM_WORDS {
            let x = self.internal[i];
            let y = rhs.internal[i];

            // Filter between different situations
            let (new_num, new_carry) = match (carry, x == Word::MAX) {
                (false, _) => x.overflowing_add(y),
                (true, true) => (y, true),
                (true, false) => (x + 1).overflowing_add(y),
            };

            carry = new_carry;
            big_int.internal[i] = new_num;
        }

        big_int
    }
}
impl<const NUM_WORDS: usize> Sub<Self> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        // Subtract by adding
        let additive = (rhs ^ Self::MAX).increase();
        additive + self
    }
}

impl<const NUM_WORDS: usize> AddAssign<Self> for BigUInt<NUM_WORDS> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<const NUM_WORDS: usize> SubAssign<Self> for BigUInt<NUM_WORDS> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

#[test]
fn add() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN + BigUInt::MIN, BigUInt::<128>::MIN);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(40u32) + <BigUInt<128>>::from(2u32),
        <BigUInt<128>>::from(42u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(250u32) + <BigUInt<128>>::from(10u32),
        <BigUInt<128>>::from(260u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        for y in 200..300u32 {
            assert_eq!(
                <BigUInt<16>>::from(x) + <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x + y)
            );
        }
    }

    // Bigger Loop check
    for x in 10000..10100u32 {
        for y in 10000..10100u32 {
            assert_eq!(
                <BigUInt<16>>::from(x) + <BigUInt<16>>::from(y),
                <BigUInt<16>>::from(x + y)
            );
        }
    }
}
#[test]
fn sub() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN - BigUInt::MIN, BigUInt::<128>::MIN);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(42u32) - <BigUInt<128>>::from(2u32),
        <BigUInt<128>>::from(40u32)
    );

    // Overflow check
    assert_eq!(
        <BigUInt<128>>::from(260u32) - <BigUInt<128>>::from(10u32),
        <BigUInt<128>>::from(250u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        for y in 200..300u32 {
            assert_eq!(
                <BigUInt<128>>::from(x + y) - <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x)
            );
        }
    }

    // Bigger Loop check
    for x in 65500..65600u32 {
        for y in 65500..65600u32 {
            assert_eq!(
                <BigUInt<128>>::from(x + y) - <BigUInt<128>>::from(y),
                <BigUInt<128>>::from(x)
            );
        }
    }
}
