use criterion::{criterion_group, criterion_main, Criterion};
use tiny_big_uint::BigUInt;

#[cfg(not(any(feature = "16bit", feature = "32bit", feature = "64bit")))]
fn max_mul(c: &mut Criterion) {
    c.bench_function("8bit Max Multiplication", |b| {
        b.iter(|| <BigUInt<32>>::from(u128::MAX) * <BigUInt<32>>::from(u128::MAX))
    });

    c.bench_function("8bit Lower Multiplication", |b| {
        b.iter(|| <BigUInt<32>>::from(u128::MAX) * <BigUInt<32>>::from(100u32))
    });
}

#[cfg(feature = "16bit")]
fn max_mul(c: &mut Criterion) {
    c.bench_function("16bit Max Multiplication", |b| {
        b.iter(|| <BigUInt<16>>::from(u128::MAX) * <BigUInt<16>>::from(u128::MAX))
    });

    c.bench_function("16bit Lower Multiplication", |b| {
        b.iter(|| <BigUInt<16>>::from(u128::MAX) * <BigUInt<16>>::from(100u32))
    });
}

#[cfg(feature = "32bit")]
fn max_mul(c: &mut Criterion) {
    c.bench_function("32bit Max Multiplication", |b| {
        b.iter(|| <BigUInt<8>>::from(u128::MAX) * <BigUInt<8>>::from(u128::MAX))
    });

    c.bench_function("32bit Lower Multiplication", |b| {
        b.iter(|| <BigUInt<8>>::from(u128::MAX) * <BigUInt<8>>::from(100u32))
    });
}

#[cfg(feature = "64bit")]
fn max_mul(c: &mut Criterion) {
    c.bench_function("64bit Max Multiplication", |b| {
        b.iter(|| <BigUInt<4>>::from(u128::MAX) * <BigUInt<4>>::from(u128::MAX))
    });

    c.bench_function("64bit Lower Multiplication", |b| {
        b.iter(|| <BigUInt<4>>::from(u128::MAX) * <BigUInt<4>>::from(100u32))
    });
}

criterion_group!(benches, max_mul);
criterion_main!(benches);