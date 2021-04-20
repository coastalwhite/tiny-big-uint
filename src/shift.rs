use crate::{BigUInt, WORD_BITS};
use core::ops::{Shl, Shr, ShlAssign, ShrAssign};

impl<const NUM_WORDS: usize> Shl<usize> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        let word_shift = rhs / WORD_BITS;
        let bit_shift = rhs % WORD_BITS;

        let mut buffer = [0; NUM_WORDS];

        // If we have exact multiple of the bytes
        if bit_shift == 0 {
            // Just shift all the words over by a set amount
            for word_index in word_shift..NUM_WORDS {
                buffer[word_index] = self.internal[word_index - word_shift];
            }
        } else {
            // If we need to take into account some extra bit offset
            for word_index in (word_shift + 1)..NUM_WORDS {
                let left_word = self.internal[word_index - word_shift];
                let right_word = self.internal[word_index - word_shift - 1];

                buffer[word_index] =
                    (left_word << bit_shift) | (right_word >> (WORD_BITS - bit_shift));
            }

            // The last byte uses 0x00 for it's left word
            buffer[word_shift] = self.internal[0] << bit_shift;
        }

        BigUInt { internal: buffer }
    }
}

impl<const NUM_WORDS: usize> Shr<usize> for BigUInt<NUM_WORDS> {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        let word_shift = rhs / WORD_BITS;
        let bit_shift = rhs % WORD_BITS;

        let mut buffer = [0; NUM_WORDS];

        // If we have exact multiple of the bytes
        if bit_shift == 0 {
            // Just shift all the words over by a set amount
            for word_index in word_shift..NUM_WORDS {
                buffer[word_index - word_shift] = self.internal[word_index];
            }
        } else {
            // If we need to take into account some extra bit offset
            for word_index in word_shift..(NUM_WORDS - 1) {
                let left_word = self.internal[word_index + 1];
                let right_word = self.internal[word_index];

                buffer[word_index - word_shift] =
                    (left_word << (WORD_BITS - bit_shift)) | (right_word >> bit_shift);
            }

            // The last byte uses 0x00 for it's left word
            buffer[NUM_WORDS - word_shift - 1] =
                self.internal[NUM_WORDS - 1] >> (WORD_BITS - bit_shift);
        }

        BigUInt { internal: buffer }
    }
}

impl<const NUM_WORDS: usize> ShlAssign<usize> for BigUInt<NUM_WORDS> {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}
impl<const NUM_WORDS: usize> ShrAssign<usize> for BigUInt<NUM_WORDS> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}

#[test]
fn lshift() {
    // Bigger Loop check
    for x in 65500..65600u64 {
        for y in 20..32usize {
            assert_eq!(<BigUInt<128>>::from(x) << y, <BigUInt<128>>::from(x << y));
        }
    }
}

#[test]
fn rshift() {
    // Bigger Loop check
    for x in 65500..65600u64 {
        for y in 20..32usize {
            assert_eq!(<BigUInt<128>>::from(x) >> y, <BigUInt<128>>::from(x >> y));
        }
    }
}
