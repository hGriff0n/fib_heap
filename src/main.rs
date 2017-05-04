extern crate rust_algos;

use rust_algos::coll::fibheap::FibHeap;

fn main() {
    let mut heap = FibHeap::new();
    heap.push(3);
    println!("Heap Size: {}", heap.len());
    println!("Heap Top: {}", heap.peek().unwrap());
}
