extern crate blake2b as blake2b_ext;
extern crate keccak as keccak_ext;
#[macro_use]
extern crate criterion;

use blake2b_ext::*;
use criterion::Criterion;
use keccak_ext::*;

pub const BLAKE2BKEY: [u8; 32] = [0u8; 32];

pub fn keccak_into(input: &[u8], dest: &mut [u8]) {
    unsafe {
        keccak_256(dest.as_mut_ptr(), dest.len(), input.as_ptr(), input.len());
    }
}

//non-key
pub fn blake2b_into(input: &[u8], dest: &mut [u8]) {
    unsafe {
        blake2b(
            dest.as_mut_ptr(),
            dest.len(),
            input.as_ptr(),
            input.len(),
            BLAKE2BKEY.as_ptr(),
            0,
        );
    }
}

fn hash_bench(c: &mut Criterion) {
    c.bench_function("keccak_bench", |b| {
        let data = [0u8; 128];
        let mut out = [0u8; 32];
        b.iter(|| {
            keccak_into(&data, &mut out);
        })
    });


    c.bench_function("blake2b_bench", |b| {
        let data = [0u8; 128];
        let mut out = [0u8; 32];
        b.iter(|| {
            blake2b_into(&data, &mut out);
        })
    });
}

criterion_group!(benches, hash_bench);
criterion_main!(benches);
