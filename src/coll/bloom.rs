extern crate bit_vec;

use std::marker;
use std::hash::{Hash, Hasher, BuildHasher};
use std::collections::hash_map::RandomState;

/*
A bloom filter is a bit array of 'm' bits combined 3with a constant k number of different hash functions

'SimpleBloom' has no ability to distinguish whether a false positive occurs
   The value for 'k' is also hardcoded so false positive isn't minimized
 */
pub struct SimpleBloom<T: Hash> {
    bitmap: bit_vec::BitVec,
    // bits: u64,
    // k: u32,
    state: [RandomState;2],
    _ign: marker::PhantomData<T>            // Utilize the generic type so the compiler doesn't complain
}

impl<T: Hash> SimpleBloom<T> {
    #[allow(unused_variables)]
    pub fn new(size: usize, nitems: usize) -> Self {
        Self{
            bitmap: bit_vec::BitVec::from_elem(size, false),
            state: [
                RandomState::new(),
                RandomState::new()
            ],
            _ign: marker::PhantomData
        }
    }
    
    #[allow(unused_variables)]
    pub fn with_state(size: usize, nitems: usize, state: [RandomState;2]) -> Self {
        Self{
            bitmap: bit_vec::BitVec::from_elem(size, false),
            state: state,
            _ign: marker::PhantomData
        }
    }

    fn optimal_k(bits: u64, items: usize) -> u32 {
        let bits = bits as f64;
        let items = items as f64;
        let k = (bits / items * f64::ln(2.0f64)).ceil() as u32;
        cmp::max(k, 1)
    }

    fn compute_size(items: usize, fp_rate: f64) -> usize {
        let log2 = f64::consts::LN_2;
        let log2 = log2 * log2;
        ((items as f64) * f64::ln(fp_rate) / (-8.0 * log2)).ceil() as usize
    }

    // Get the hash value for the data item
    fn hash(&self, data: T) -> (usize, usize) {
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

    // Wrapper for checking whether the bitmap is set
    fn is_set(&self, idx: usize) -> bool {
        self.bitmap.get(idx).unwrap_or(false)
    }

    // Adds a record of the item to the bloom filter
    pub fn add(&mut self, data: T) {
        let (h1, h2) = self.hash(data);
        self.bitmap.set(h1, true);
        self.bitmap.set(h2, true);
    }

    // Test whether the item is recorded within the filter
    pub fn contains(&self, data: T) -> bool {
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