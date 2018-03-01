extern crate jump_hash;
extern crate seahash;

use jump_hash::{slot, JumpHash};
use seahash::SeaHasher;

#[derive(PartialEq, Hash)]
struct Person {
    name: &'static str,
    age: u8,
}

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

struct Test {
    key: u64,
    len: Vec<u32>,
}

#[test]
fn table_test() {
    let tests = vec![
        Test {
            key: 0,
            len: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        },
        Test {
            key: 1,
            len: vec![0, 0, 0, 0, 0, 0, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 6, 17, 17],
        },
        Test {
            key: 0xdeadbeef,
            len: vec![0, 1, 2, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 16, 16, 16],
        },
        Test {
            key: 0x0ddc0ffeebadf00d,
            len: vec![0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 15, 15, 15, 15],
        },
    ];
    for test in tests {
        for (i, &len) in test.len.iter().enumerate() {
            let got = slot(test.key, i + 1);
            assert_eq!(got, len);
        }
    }
}

#[test]
fn slot_test() {
    assert_eq!(6, slot(10863919174838991, 11));
    assert_eq!(3, slot(2016238256797177309, 11));
    assert_eq!(5, slot(1673758223894951030, 11));
    assert_eq!(80343, slot(2, 100001));
    assert_eq!(22152, slot(2201, 100001));
    assert_eq!(15018, slot(2202, 100001));
}

#[test]
fn hasher_test() {
    let state = BuildSeaHasher::new(
        0xe7b0c93ca8525013,
        0x011d02b854ae8182,
        0x7bcc5cf9c39cec76,
        0xfa336285d102d083,
    );
    let hash1 = JumpHash::with_hasher(11, state);
    let hash2 = JumpHash::with_hasher(10, state);
    let hash3 = JumpHash::with_hasher(11, state);

    let alice = &Person {
        name: "alice",
        age: 20,
    };
    let bob = &Person {
        name: "bob",
        age: 30,
    };

    assert_eq!(7, hash1.get(alice));
    assert_eq!(6, hash1.get(bob));

    assert_eq!(7, hash2.get(alice));
    assert_eq!(6, hash2.get(bob));

    assert_eq!(7, hash3.get(alice));
    assert_eq!(6, hash3.get(bob));
}
