use crate::{BigUInt, Word, WORD_BITS};

macro_rules! upcast_type_bigger_than_word {
    ($utype:ty) => {
        impl<const NUM_WORDS: usize> From<$utype> for BigUInt<NUM_WORDS> {
            fn from(mut num: $utype) -> Self {
                let mut internal = [0; NUM_WORDS];

                let mut index = 0;
                while num != 0 {
                    // Copy over the first X bits
                    internal[index] = (num & <$utype>::from(Word::MAX)) as Word;
                    // Shift over X bits
                    num >>= WORD_BITS;

                    index += 1;
                }

                BigUInt { internal }
            }
        }
    };
}

macro_rules! upcast_type_smaller_eq_than_word {
    ($utype:ty) => {
        impl<const NUM_WORDS: usize> From<$utype> for BigUInt<NUM_WORDS> {
            fn from(num: $utype) -> Self {
                let mut internal = [0; NUM_WORDS];
                internal[0] = Word::from(num);

                BigUInt { internal }
            }
        }
    };
}

upcast_type_smaller_eq_than_word!(u8);

#[cfg(any(feature = "16bit", feature = "32bit", feature = "64bit"))]
upcast_type_smaller_eq_than_word!(u16);
#[cfg(not(any(feature = "16bit", feature = "32bit", feature = "64bit")))]
upcast_type_bigger_than_word!(u16);

#[cfg(any(feature = "32bit", feature = "64bit"))]
upcast_type_smaller_eq_than_word!(u32);
#[cfg(not(any(feature = "32bit", feature = "64bit")))]
upcast_type_bigger_than_word!(u32);

#[cfg(feature = "64bit")]
upcast_type_smaller_eq_than_word!(u64);
#[cfg(not(feature = "64bit"))]
upcast_type_bigger_than_word!(u64);

upcast_type_bigger_than_word!(u128);

macro_rules! to_test {
    ($type:ty, $name:ident) => {
        #[test]
        fn $name() {
            assert_eq!(<BigUInt<16>>::from(<$type>::MIN).internal, [0; 16]);

            for i in (<$type>::MAX - 100)..<$type>::MAX {
                let mut arr = [0; 16];
                for (index, x) in i.to_be_bytes().iter().rev().enumerate() {
                    arr[15 - index] = *x;
                }
                
                assert_eq!(
                    <BigUInt<16>>::from(i),
                    <BigUInt<16>>::from_be_bytes(arr)
                );
            }
        }
    };
}

to_test!(u8, from_u8);
to_test!(u16, from_u16);
to_test!(u32, from_u32);
to_test!(u64, from_u64);
to_test!(u128, from_u128);
