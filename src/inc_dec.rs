use crate::{BigUInt, Word};

impl<const NUM_WORDS: usize> BigUInt<NUM_WORDS> {
    /// Increase a [`BigUInt`](crate::BigUInt) by 1.
    ///
    /// Will overflow back to [`MIN`](crate::BigUInt::MIN), when ran on the
    /// [`MAX`](crate::BigUInt::MIN) value.
    pub fn increase(mut self) -> Self {
        // Go until a byte has been found that 1 can be added to.
        for i in 0..NUM_WORDS {
            let byte = self.internal[i];

            // Check whether the value can be added to.
            if byte < Word::MAX {
                self.internal[i] = byte + 1;
                break;
            } else {
                self.internal[i] = 0;
            }
        }

        self
    }

    /// Decrease a [`BigUInt`](crate::BigUInt) by 1.
    ///
    /// Will underflow back to [`MAX`](crate::BigUInt::MAX), when ran on the
    /// [`MIN`](crate::BigUInt::MAX) value.
    pub fn decrease(mut self) -> Self {
        // Go until a byte has been found that 1 can be subtracted from.
        for i in 0..NUM_WORDS {
            let byte = self.internal[i];

            // Check whether the value can be subtracted from.
            if byte > 0 {
                self.internal[i] = byte - 1;
                break;
            } else {
                self.internal[i] = Word::MAX;
            }
        }

        self
    }
}

#[test]
fn increase() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN.increase(), BigUInt::<128>::from(1u32));

    // Check overflow behavior
    assert_eq!(BigUInt::MAX.increase(), BigUInt::<32>::MIN);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(41u32).increase(),
        <BigUInt<128>>::from(42u32)
    );

    // Word overflow check
    assert_eq!(
        <BigUInt<128>>::from(255u32).increase(),
        <BigUInt<128>>::from(256u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        assert_eq!(<BigUInt<128>>::from(x).increase(), <BigUInt<128>>::from(x + 1));
    }

    // Bigger Loop check
    for x in 10000..10100u32 {
        assert_eq!(<BigUInt<128>>::from(x).increase(), <BigUInt<128>>::from(x + 1));
    }
}
#[test]
fn decrease() {
    // Check it stays 0
    assert_eq!(BigUInt::MIN.decrease(), BigUInt::<128>::MAX);

    // Check underflow behavior
    assert_eq!(BigUInt::MIN.decrease(), BigUInt::<32>::MAX);

    // Basic math check
    assert_eq!(
        <BigUInt<128>>::from(42u32).decrease(),
        <BigUInt<128>>::from(41u32)
    );

    // Word overflow check
    assert_eq!(
        <BigUInt<128>>::from(256u32).decrease(),
        <BigUInt<128>>::from(255u32)
    );

    // Small Loop check
    for x in 200..300u32 {
        assert_eq!(<BigUInt<128>>::from(x).decrease(), <BigUInt<128>>::from(x - 1));
    }

    // Bigger Loop check
    for x in 10000..10100u32 {
        assert_eq!(<BigUInt<128>>::from(x).decrease(), <BigUInt<128>>::from(x - 1));
    }
}
