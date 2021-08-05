# Tiny Big UInt

<!-- cargo-sync-readme start -->

An minimal and tiny, no heap, no_std implementation for big unsigned integers using
const generics. So you can do arithmetic on all numbers on all devices!

```rust
use tiny_big_uint::BigUInt;

<BigUInt<32>>::from(u128::MAX) * <BigUInt<32>>:from(u128::MAX)
```

This library allows for [`u8`](https://doc.rust-lang.org/stable/core/u8/), [`u16`](https://doc.rust-lang.org/stable/core/u16/), [`u32`](https://doc.rust-lang.org/stable/core/u32/), or
[`u64`](https://doc.rust-lang.org/stable/core/u64/) words sizes with the *default*, *16bit*, *32bit* and *64bit* features,
respectively. By default, it will use a [`u8`](https://doc.rust-lang.org/stable/core/u8/) word size. Being able to change the
word size, can be especially handy when working on very specific hardware.

# Why use this crate?

This crate is useful for when you don't want to utitize dynamic memory allocation, but still need bigger than 128
bit unsigned integers. Mostly, this is used on low powered embedded hardware for example with
encryption algorithms. Since const generics are now officially out, I felt it necessary to put
that feature to good use.

# Alternatives

* [Tiny-Bignum for C by kokke](https://github.com/kokke/tiny-bignum-c) - Implemented in C
* [smallbigint by Bram Geron](https://github.com/rusty-plasma/smallbigint) - Uses the heap
* [num-bigint by rust-num](https://github.com/rust-num/num-bigint) - Uses the heap

# Usage

This library provides one type: [`BigUInt`](https://docs.rs/tiny-big-uint/latest/tiny-big-uint/struct.BigUInt.html). The size for this type is
completely determined compile time and thus is allocated on the stack. It basically implements
all the normal arithmetic operators for this `struct`. Along with some other helpful functions,
most of them packed under their own features.

# Features

## Creation from standard unsigned integers

The *upcasting* feature allows for initiation of a [`BigUInt`](https://docs.rs/tiny-big-uint/latest/tiny-big-uint/struct.BigUInt.html) from the
standard [`u8`](https://doc.rust-lang.org/stable/core/u8/), [`u16`](https://doc.rust-lang.org/stable/core/u16/),
[`u32`](https://doc.rust-lang.org/stable/core/u32/), [`u64`](https://doc.rust-lang.org/stable/core/u64/) and [`u128`](https://doc.rust-lang.org/stable/core/u128/)
types.

### Example

```rust
use tiny_big_uint::BigUInt;

// Create new BigUInt from u32
<BigUInt<32>>::from(1234u32);
```

To use this, just add the following to your `Cargo.toml`.

```toml
[dependencies]
tiny-big-uint = { version = "^1.0", features = ["upcasting"] }
```

## Creation from byte arrays

The *bytearrays* feature allows for initiation of a [`BigUInt`](https://docs.rs/tiny-big-uint/latest/tiny-big-uint/struct.BigUInt.html) from either a
Little-Endian or Big-Endian byte-array. This is especially useful when working with 16 or 32
bit systems.

### Example

```rust
use tiny_big_uint::BigUInt;

let a = BigUInt::from_be_bytes([0x00, 0x76, 0x54, 0x32, 0x10]);
assert_eq!(a << 4, BigUInt::from_be_bytes([0x07, 0x65, 0x43, 0x21, 0x00]));
```

To use this, just add the following to your `Cargo.toml`.

```toml
[dependencies]
tiny-big-uint = { version = "^1.0", features = ["bytearrays"] }
```

## Downcasting to standard types

In order to cast back to standard types, one can use the *downcasting* feature. This feature
allows for one to cast a number back to [`u8`](https://doc.rust-lang.org/stable/core/u8/),
[`u16`](https://doc.rust-lang.org/stable/core/u16/), [`u32`](https://doc.rust-lang.org/stable/core/u32/), [`u64`](https://doc.rust-lang.org/stable/core/u64/) and
[`u128`](https://doc.rust-lang.org/stable/core/u128/).

### Example

```rust
use tiny_big_uint::BigUInt;

let a = BigUInt::from_be_words([0x00, 0x76, 0x54, 0x32, 0x10]);
assert_eq!(a.to_u128().unwrap(), 0x76543210);
```

To use this, just add the following to your `Cargo.toml`.

```toml
[dependencies]
tiny-big-uint = { version = "^1.0", features = ["downcasting"] }
```

<!-- cargo-sync-readme end -->

## License

Licensed under either a MIT or Apache License.
