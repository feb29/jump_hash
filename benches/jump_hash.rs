#![feature(test)]

extern crate jump_hash;
extern crate seahash;
extern crate test;

use test::Bencher;
use jump_hash::JumpHash;
use seahash::SeaHasher;

#[derive(Debug, Clone, Copy)]
struct BuildSeaHasher {
    seeds: (u64, u64, u64, u64),
}
impl BuildSeaHasher {
    fn new(s1: u64, s2: u64, s3: u64, s4: u64) -> Self {
        Self {
            seeds: (s1, s2, s3, s4),
        }
    }
}

impl ::std::hash::BuildHasher for BuildSeaHasher {
    type Hasher = SeaHasher;
    fn build_hasher(&self) -> Self::Hasher {
        let (s1, s2, s3, s4) = self.seeds;
        SeaHasher::with_seeds(s1, s2, s3, s4)
    }
}

macro_rules! bench_slot {
    ( $b:expr, $n:expr ) => {
        {
            let jh = jump_hash::JumpHash::new($n);
            $b.iter(|| test::black_box(jh.get(b"madoka")));
        }
    }
}

#[bench]
fn sea_hasher(b: &mut Bencher) {
    let state = BuildSeaHasher::new(
        0xe7b0c93ca8525013,
        0x011d02b854ae8182,
        0x7bcc5cf9c39cec76,
        0xfa336285d102d083,
    );
    let jh = JumpHash::with_hasher(11, state);
    b.iter(|| test::black_box(jh.get(b"alice")));
}

#[bench]
fn random_state(b: &mut Bencher) {
    let jh = JumpHash::new(11);
    b.iter(|| test::black_box(jh.get(b"alice")));
}

#[bench]
fn slot_8(b: &mut Bencher) {
    bench_slot!(b, 8)
}
#[bench]
fn slot_32(b: &mut Bencher) {
    bench_slot!(b, 32)
}
#[bench]
fn slot_128(b: &mut Bencher) {
    bench_slot!(b, 128)
}
#[bench]
fn slot_512(b: &mut Bencher) {
    bench_slot!(b, 512)
}
#[bench]
fn slot_1024(b: &mut Bencher) {
    bench_slot!(b, 1024)
}
#[bench]
fn slot_2048(b: &mut Bencher) {
    bench_slot!(b, 2048)
}
#[bench]
fn slot_65536(b: &mut Bencher) {
    bench_slot!(b, 65536)
}
