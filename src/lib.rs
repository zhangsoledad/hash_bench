#![feature(test)]
extern crate test;
extern crate sha3 as sha3_ext;
extern crate blake2b as blake2b_ext;

use sha3_ext::*;
use blake2b_ext::*;
use test::Bencher;

pub const BLAKE2BKEY: [u8; 32] = [0u8; 32];

pub fn sha3_into(input: &[u8], dest: &mut [u8]) {
    unsafe {
        sha3_256(dest.as_mut_ptr(), dest.len(), input.as_ptr(), input.len());
    }
}

pub fn blake2b_into(input: &[u8], dest: &mut [u8]) {
    unsafe {
        blake2b(dest.as_mut_ptr(), dest.len(), input.as_ptr(), input.len(), BLAKE2BKEY.as_ptr(), 32);
    }
}

#[bench]
fn sha3_bench(b: &mut Bencher) {
    let mut data = [0u8; 128];
    let mut out = [0u8; 32];	
    b.iter(|| {
        for _ in 0..999 {
            sha3_into(&data, &mut out);
        }
    });
}

#[bench]
fn blake2b_bench(b: &mut Bencher) {
    let mut data = [0u8; 128];
    let mut out = [0u8; 32];
    b.iter(|| {
        for _ in 0..999 {
            blake2b_into(&data, &mut out);
        }
    });
}
