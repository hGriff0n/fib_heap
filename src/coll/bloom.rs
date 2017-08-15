extern crate bit_vec;

use std::marker;
use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::RandomState;

/*
A bloom filter is a bit array of 'm' bits combined 3with a constant k number of different hash functions

'SimpleBloom' has no ability to distinguish whether a false positive occurs
   The value for 'k' is also hardcoded so false positive isn't minimized
 */
 // TODO: Genericize
pub struct SimpleBloom {
    bitmap: bit_vec::BitVec,
    // bits: u64,
    // k: u32,
    // sips: [SipHashser13;2],
    state: [RandomState;2],
    // _ign: marker::PhantomData<T>            // Utilize the generic type so the compiler doesn't complain
}

impl SimpleBloom {
    pub fn new(size: usize, nitems: usize) -> Self {
        Self{
            bitmap: bit_vec::BitVec::from_elem(size, false),
            state: [
                RandomState::new(),
                RandomState::new()
            ]
            // _ign: marker::PhantomData
        }
    }

    fn hash(&self, data: i32) -> (usize, usize) {
        let h1 = {
            let mut hash = self.state[0].build_hasher();
            data.hash(&mut hash);
            hash.finish() as usize % self.bitmap.len()
        };
        let h2 = {
            let mut hash = self.state[1].build_hasher();
            data.hash(&mut hash);
            hash.finish() as usize % self.bitmap.len()
        };
        (h1 as usize, h2 as usize)
    }

    fn is_set(&self, idx: usize) -> bool {
        self.bitmap.get(idx).unwrap_or(false)
    }

    // Adds a record of the item to the bloom filter
    pub fn add(&mut self, data: i32) {
        let (h1, h2) = self.hash(data);
        self.bitmap.set(h1, true);
        self.bitmap.set(h2, true);
    }

    // Test whether the item is recorded within the filter
    pub fn contains(&self, data: i32) -> bool {
        let (h1, h2) = self.hash(data);
        self.is_set(h1) && self.is_set(h2)
    }

    // Clear out the filter (if necessary)
    pub fn clear(&mut self) {
        self.bitmap.clear()
    }

    // How many bits are in the filter
    pub fn len(&self) -> usize {
        self.bitmap.len()
    }
}