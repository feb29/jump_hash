//! Implements 'Jump Consistent Hash' from the paper
//! [A Fast, Minimal Memory, Consistent Hash Algorithm](http://arxiv.org/abs/1406.2294)
//! by John Lamping, Eric Veach (2014).

#![deny(warnings)]

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

const JUMP: u64 = 1 << 31;

/// Takes a 64 bit key and the number of slot, outputs a slot number `[0, slot)`.
///
/// # Examples
///
/// ```
/// extern crate jump_hash;
/// assert_eq!(jump_hash::slot(0, 60), 0);
/// assert_eq!(jump_hash::slot(1, 60), 55);
/// assert_eq!(jump_hash::slot(2, 60), 46);
/// ```
pub fn slot(mut k: u64, mut n: usize) -> u32 {
    if n == 0 {
        n = 1;
    }
    let mut b = -1;
    let mut j = 0;
    while j < n as i64 {
        b = j;
        k = k.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ((b + 1) as f64 * (JUMP as f64 / ((k >> 33) + 1) as f64)) as i64;
    }
    b as u32
}

pub struct JumpHash<S = RandomState> {
    pub slots: usize,
    state: S,
}

impl JumpHash {
    pub fn new(slots: usize) -> Self {
        JumpHash {
            slots,
            state: Default::default(),
        }
    }
}

impl<S> JumpHash<S>
where
    S: BuildHasher,
{
    pub fn with_hasher(slots: usize, state: S) -> Self {
        JumpHash { slots, state }
    }

    /// Takes a key, outputs a number `0..slots`.
    pub fn get<T: Hash + ?Sized>(&self, key: &T) -> u32 {
        let hasher = &mut self.state.build_hasher();
        key.hash(hasher);
        slot(hasher.finish(), self.slots)
    }
}
