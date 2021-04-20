use crate::BigUInt;

// The type attempting cast to is bigger than the word size
macro_rules! downcast_type_bigger_than_word {
    ($type:ty, $fn_name:ident) => {
        impl<const NUM_WORDS: usize> BigUInt<NUM_WORDS> {
            /// Attempt to downcast to specified type.
            ///
            /// If value exceeds maximum value of type to which cast was attempted, the return
            /// value will be `None`.
            pub fn $fn_name(self) -> Option<$type> {
                if !(self.clone() >> (core::mem::size_of::<$type>() * 8)).is_zero() {
                    return None;
                }
                const WORDS_NEEDED: usize = core::mem::size_of::<$type>() / crate::WORD_BYTES;
                let mut num: $type = 0;

                for i in 0..crate::min(WORDS_NEEDED, NUM_WORDS) {
                    num |= (<$type>::from(self.internal[i]) << (i * crate::WORD_BITS));
                }

                Some(num)
            }
        }
    };
}

// The type attempting cast to is smaller than of equal to the word size
macro_rules! downcast_type_smaller_eq_than_word {
    ($type:ty, $fn_name:ident) => {
        impl<const NUM_WORDS: usize> BigUInt<NUM_WORDS> {
            /// Attempt to downcast to specified type.
            ///
            /// If value exceeds maximum value of type to which cast was attempted, the return
            /// value will be `None`.
            pub fn $fn_name(self) -> Option<$type> {
                if !(self.clone() >> (core::mem::size_of::<$type>() * 8)).is_zero() {
                    return None;
                }

                Some(<$type>::MAX & (self.internal[0] as $type))
            }
        }
    };
}

downcast_type_smaller_eq_than_word!(u8, to_u8);
downcast_type_bigger_than_word!(u128, to_u128);

#[cfg(any(feature = "16bit", feature = "32bit", feature = "64bit"))]
downcast_type_smaller_eq_than_word!(u16, to_u16);
#[cfg(not(any(feature = "16bit", feature = "32bit", feature = "64bit")))]
downcast_type_bigger_than_word!(u16, to_u16);

#[cfg(any(feature = "32bit", feature = "64bit"))]
downcast_type_smaller_eq_than_word!(u32, to_u32);
#[cfg(not(any(feature = "32bit", feature = "64bit")))]
downcast_type_bigger_than_word!(u32, to_u32);

#[cfg(feature = "64bit")]
downcast_type_smaller_eq_than_word!(u64, to_u64);
#[cfg(not(feature = "64bit"))]
downcast_type_bigger_than_word!(u64, to_u64);

macro_rules! to_test {
    ($type:ty, $fn_name:ident) => {
        #[test]
        fn $fn_name() {
            assert_eq!(<BigUInt<20>>::from(0u8).$fn_name(), Some(0));

            for i in <$type>::MAX - 100..=<$type>::MAX {
                assert_eq!(<BigUInt<20>>::from(i).$fn_name(), Some(i));
            }

            assert_eq!(
                <BigUInt<20>>::from(<$type>::MAX).increase().$fn_name(),
                None
            );
        }
    };
}

to_test!(u8, to_u8);
to_test!(u16, to_u16);
to_test!(u32, to_u32);
to_test!(u64, to_u64);
to_test!(u128, to_u128);
