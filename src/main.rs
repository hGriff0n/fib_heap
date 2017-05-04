extern crate rust_algos;

use rust_algos::coll::fibheap::FibHeap;

fn main() {
    let mut heap = FibHeap::new();
    // let mut heap: FibHeap<i32> = FibHeap::new();
    
    heap.push(3);
    println!("Heap Size: {}", heap.len());
    println!("Heap Top: {:?}", heap.peek());
    println!("-----------------------");

    heap.push(5);
    println!("Heap Size: {}", heap.len());
    println!("Heap Top: {:?}", heap.peek());
    println!("-----------------------");

    heap.push(2);
    println!("Heap Size: {}", heap.len());
    println!("Heap Top: {:?}", heap.peek());
    println!("-----------------------");

    println!("Pop Heap: {:?}", heap.pop());
    println!("Heap Size: {}", heap.len());
    println!("Heap Top: {:?}", heap.peek());
}
