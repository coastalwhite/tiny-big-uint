use crate::BigUInt;

// If the words are bytes, these can simply link to the from from_..._words functions.
#[cfg(not(any(feature = "16bit", feature = "32bit", feature = "64bit")))]
impl<const NUM_WORDS: usize> BigUInt<NUM_WORDS> {
    /// Create a [`BigUInt`](crate::BigUInt) from a byte array in Little-Endian order.
    pub fn from_le_bytes(bytes: [u8; NUM_WORDS]) -> Self {
        Self::from_le_words(bytes)
    }

    /// Create a [`BigUInt`](crate::BigUInt) from a byte array in Big-Endian order.
    pub fn from_be_bytes(bytes: [u8; NUM_WORDS]) -> Self {
        Self::from_be_words(bytes)
    }
}

#[cfg(any(feature = "16bit", feature = "32bit", feature = "64bit"))]
impl<const NUM_WORDS: usize> BigUInt<NUM_WORDS> {
    /// Create a [`BigUInt`](crate::BigUInt) from a byte array in
    /// [Little-Endian](https://en.wikipedia.org/wiki/Endianness) order.
    ///
    /// Any leftover bytes will not be used. Any missing bytes will be seen as `0x00`.
    pub fn from_le_bytes<const NUM_BYTES: usize>(bytes: [u8; NUM_BYTES]) -> Self {
        let mut le_words = [0; NUM_WORDS];

        let mut num = 0;
        for i in 0..crate::min(crate::WORD_BYTES * NUM_WORDS, NUM_BYTES) {
            num |= <crate::Word>::from(bytes[i]) << ((i % crate::WORD_BYTES) * 8);

            if (i + 1) % crate::WORD_BYTES == 0 {
                le_words[i / crate::WORD_BYTES] = num;
                num = 0;
            }
        }

        Self::from_le_words(le_words)
    }

    /// Create a [`BigUInt`](crate::BigUInt) from a byte array in
    /// [Big-Endian](https://en.wikipedia.org/wiki/Endianness) order.
    pub fn from_be_bytes<const NUM_BYTES: usize>(bytes: [u8; NUM_BYTES]) -> Self {
        let mut be_words = [0; NUM_WORDS];

        let mut num = 0;
        for i in 0..crate::min(crate::WORD_BYTES * NUM_WORDS, NUM_BYTES) {
            num <<= 8;
            num |= <crate::Word>::from(bytes[i]);

            if (i + 1) % crate::WORD_BYTES == 0 {
                be_words[i / crate::WORD_BYTES] = num;
                num = 0;
            }
        }

        Self::from_be_words(be_words)
    }
}

#[cfg(not(any(feature = "16bit", feature = "32bit", feature = "64bit")))]
#[test]
fn from_be_bytes() {
    assert_eq!(
        <BigUInt<16>>::from_be_bytes([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x89, 0x67, 0x45,
            0x23, 0x01
        ])
        .to_u128()
        .unwrap(),
        0x8967452301
    );
}
#[cfg(feature = "16bit")]
#[test]
fn from_be_bytes() {
    assert_eq!(
        <BigUInt<2>>::from_be_bytes([0x67, 0x45, 0x23, 0x01])
            .to_u128()
            .unwrap(),
        0x67452301
    );
}
#[cfg(feature = "32bit")]
#[test]
fn from_be_bytes() {
    assert_eq!(
        <BigUInt<1>>::from_be_bytes([0x67, 0x45, 0x23, 0x01])
            .to_u128()
            .unwrap(),
        0x67452301
    );
}
#[cfg(feature = "64bit")]
#[test]
fn from_be_bytes() {
    assert_eq!(
        <BigUInt<1>>::from_be_bytes([0x00, 0x00, 0x00, 0x00, 0x67, 0x45, 0x23, 0x01])
            .to_u128()
            .unwrap(),
        0x67452301
    );
}

#[test]
fn from_le_bytes() {
    assert_eq!(
        <BigUInt<16>>::from_le_bytes([
            0x01, 0x23, 0x45, 0x67, 0x89, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00
        ])
        .to_u128()
        .unwrap(),
        0x8967452301
    );
}
