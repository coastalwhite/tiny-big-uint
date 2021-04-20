//! An minimal and tiny, no heap, no_std implementation for big unsigned integers using
//! const generics. So you can do arithmetic on all numbers on all devices!
//!
//! ```rust
//! use tiny_big_uint::BigUInt;
//!
//! <BigUInt<32>>::from(u128::MAX) * <BigUInt<32>>::from(u128::MAX);
//! ```
//!
//! This library allows for [`u8`](::core::u8), [`u16`](::core::u16), [`u32`](::core::u32), or
//! [`u64`](::core::u64) words sizes with the *default*, *16bit*, *32bit* and *64bit* features,
//! respectively. By default, it will use a [`u8`](::core::u8) word size. Being able to change the
//! word size, can be especially handy when working on very specific hardware.
//!
//! # Why use this crate?
//!
//! This crate is useful for when you don't want to utitize dynamic, but still need bigger than 128
//! bit unsigned integers. Mostly, this is used on low powered embedded hardware for example with
//! encryption algorithms. Since const generics are now officially out, I felt it necessary to put
//! that feature to good use.
//!
//! # Alternatives
//!
//! * [Tiny-Bignum for C by kokke](https://github.com/kokke/tiny-bignum-c) - Implemented in C
//! * [smallbigint by Bram Geron](https://github.com/rusty-plasma/smallbigint) - Uses the heap
//! * [num-bigint by rust-num](https://github.com/rust-num/num-bigint) - Uses the heap
//!
//! # Usage
//!
//! This library provides one type: [`BigUInt`](crate::BigUInt). The size for this type is
//! completely determined compile time and thus is allocated on the stack. It basically implements
//! all the normal arithmetic operators for this `struct`. Along with some other helpful functions,
//! most of them packed under their own features.
//!
//! # Features
//!
//! ## Creation from standard unsigned integers
//!
//! The *upcasting* feature allows for initiation of a [`BigUInt`](crate::BigUInt) from the
//! standard [`u8`](::core::u8), [`u16`](::core::u16),
//! [`u32`](::core::u32), [`u64`](::core::u64) and [`u128`](::core::u128)
//! types.
//!
//! ### Example
//!
//! ```rust
//! use tiny_big_uint::BigUInt;
//!
//! // Create new BigUInt from u32
//! <BigUInt<32>>::from(1234u32);
//! ```
//!
//! To use this, just add the following to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! tiny-big-uint = { version = "^1.0", features = ["upcasting"] }
//! ```
//!
//! ## Creation from byte arrays
//!
//! The *bytearrays* feature allows for initiation of a [`BigUInt`](crate::BigUInt) from either a
//! Little-Endian or Big-Endian byte-array. This is especially useful when working with 16 or 32
//! bit systems.
//!
//! ### Example
//!
//! ```rust
//! use tiny_big_uint::BigUInt;
//!
//! let a = BigUInt::from_be_bytes([0x00, 0x76, 0x54, 0x32, 0x10]);
//! assert_eq!(a << 4, BigUInt::from_be_bytes([0x07, 0x65, 0x43, 0x21, 0x00]));
//! ```
//!
//! To use this, just add the following to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! tiny-big-uint = { version = "^1.0", features = ["bytearrays"] }
//! ```
//!
//! ## Downcasting to standard types
//!
//! In order to cast back to standard types, one can use the *downcasting* feature. This feature
//! allows for one to cast a number back to [`u8`](::core::u8),
//! [`u16`](::core::u16), [`u32`](::core::u32), [`u64`](::core::u64) and
//! [`u128`](::core::u128).
//!
//! ### Example
//!
//! ```rust
//! use tiny_big_uint::BigUInt;
//!
//! let a = BigUInt::from_be_words([0x00, 0x76, 0x54, 0x32, 0x10]);
//! assert_eq!(a.to_u128().unwrap(), 0x76543210);
//! ```
//!
//! To use this, just add the following to your `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! tiny-big-uint = { version = "^1.0", features = ["downcasting"] }
//! ```

#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

mod add_sub;
mod bitwise_ops;
#[cfg(any(feature = "bytearrays", test))]
mod bytearrays;
#[cfg(any(feature = "upcasting", test))]
mod upcast;
#[cfg(any(feature = "downcasting", test))]
mod downcast;
mod inc_dec;
mod mul_div_rem;
mod shift;

use core::mem::size_of;

#[cfg(any(
    all(feature = "16bit", any(feature = "32bit", feature = "64bit")),
    all(feature = "32bit", feature = "64bit")
))]
compile_error!("Unable to combine different word-sizes for BigUInt!");

#[cfg(not(any(feature = "16bit", feature = "32bit", feature = "64bit")))]
pub(crate) type Word = u8;
#[cfg(feature = "16bit")]
pub(crate) type Word = u16;
#[cfg(feature = "32bit")]
pub(crate) type Word = u32;
#[cfg(feature = "64bit")]
pub(crate) type Word = u64;

pub(crate) const WORD_BYTES: usize = size_of::<Word>();
pub(crate) const WORD_BITS: usize = WORD_BYTES * 8;

#[cfg(any(feature = "downcasting", feature = "bytearrays", test))]
pub(crate) const fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

/// An arbitrarily sized unsigned integer.
///
/// A representation of an unsigned integer containing a specified amount of words of which the
/// size can be configured with features (default being 8-bit).
///
/// # Examples
///
/// ```
/// use tiny_big_uint::BigUInt;
///
/// let a = BigUInt::from_be_words([0x00, 0x76, 0x54, 0x32, 0x10]);
///
/// assert_eq!(a << 4, BigUInt::from_be_words([0x07, 0x65, 0x43, 0x21, 0x00]));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUInt<const NUM_WORDS: usize> {
    /// Little-endian representation of bytes of the bytes of a bigger unsigned integer
    internal: [Word; NUM_WORDS],
}

impl<const NUM_WORDS: usize> BigUInt<NUM_WORDS> {
    /// A minimally valued BigUInt, meaning it being valued 0
    pub const MIN: Self = BigUInt {
        internal: [0; NUM_WORDS],
    };

    /// A maximally valued BigUInt, meaning it being valued `2^(NUM_WORDS*size_of(WORD)*8) - 1`
    pub const MAX: Self = BigUInt {
        internal: [Word::MAX; NUM_WORDS],
    };

    /// Create a BigUInt from words put in Little-Endian
    pub fn from_le_words(words: [Word; NUM_WORDS]) -> Self {
        BigUInt { internal: words }
    }

    /// Create a BigUInt from words put in Big-Endian
    pub fn from_be_words(words: [Word; NUM_WORDS]) -> Self {
        let mut internal = [0; NUM_WORDS];

        // Invert the words
        for i in 0..NUM_WORDS {
            internal[i] = words[NUM_WORDS - i - 1];
        }

        BigUInt { internal }
    }

    /// Returns whether a BigUInt equals 0
    pub fn is_zero(&self) -> bool {
        for i in 0..NUM_WORDS {
            if self.internal[i] != 0 {
                return false;
            }
        }

        true
    }
}

use core::cmp::{Ord, Ordering};

impl<const NUM_WORDS: usize> PartialOrd for BigUInt<NUM_WORDS> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const NUM_WORDS: usize> Ord for BigUInt<NUM_WORDS> {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..NUM_WORDS).rev() {
            match self.internal[i].cmp(&other.internal[i]) {
                Ordering::Equal => continue,
                x => return x,
            }
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bigint_init() {
        assert_eq!(BigUInt::MIN.internal, [0; 128]);
        assert_eq!(BigUInt::MIN.internal, [0; 256]);
    }

    #[test]
    fn bigint_cmp() {
        for x in 200..300u32 {
            for y in 200..300u32 {
                assert_eq!(x < y, <BigUInt<16>>::from(x) < BigUInt::from(y));
            }
        }
    }
    #[test]
    fn bigint_equal() {
        for x in 200..300u32 {
            for y in 200..300u32 {
                assert_eq!(x == y, <BigUInt<16>>::from(x) == BigUInt::from(y));
            }
        }
    }
    #[test]
    fn bigint_is_zero() {
        assert!(<BigUInt<128>>::MIN.is_zero());
        assert!(<BigUInt<128>>::from(0u32).is_zero());
        assert!(!<BigUInt<128>>::from(1u32).is_zero());
        assert!(!<BigUInt<128>>::from(321u32).is_zero());
    }
}
