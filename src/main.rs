extern crate rust_algos;

use rust_algos::*;
use coll::*;

fn main() {
    let mut heap = SimpleBloom::new(32, 10);
    
    heap.add(13);
    heap.add(15);

    println!("{}", heap.contains(12));
    println!("{}", heap.contains(13));

    // TODO: Adjust hashes to figure out and show a false positive
}