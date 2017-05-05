extern crate rust_algos;

use rust_algos::coll::fibheap::FibHeap;

fn main() {
    let mut heap = FibHeap::new();
    // let mut heap: FibHeap<i32> = FibHeap::new();
    
    heap.push(-1);
    heap.push(-4);
    heap.push(-3);

    // #size: 3, [(elem: Some(-1), sub: []), (elem: Some(-4), sub: []), (elem: Some(-3), sub: [])])]
    println!("{:?}", heap);
    println!("");

    // Heap Top: Some(-1)
    println!("Heap Top: {:?}", heap.pop());
    // #size: 2, [(elem: Some(-3), sub: [(elem: Some(-4), sub: [])])]
    println!("{:?}", heap);
    println!("");

    heap.push(-2);
    heap.push(-5);
    heap.push(-6);

    // #size: 5, [(elem: Some(-3), sub: [(elem: Some(-4), sub: [])]), (elem: Some(-2), sub: []), (elem: Some(-5), sub: []), (elem: Some(-6), sub: [])]
    println!("{:?}", heap);
    println!("");

    // Heap Top: Some(-2)
    println!("Heap Top: {:?}", heap.pop());
    // #size: 4, [(elem: Some(-3), sub: [(elem: Some(-4), sub: []), (elem: Some(-5), sub: [(elem: Some(-6), sub: [])])])]
    println!("{:?}", heap);
    println!("");

    // Heap Top: Some(-3)
    println!("Heap Top: {:?}", heap.pop());
    // #size: 3, [(elem: Some(-5), sub: [(elem: Some(-6), sub: [])]), (elem: Some(-4), sub: [])]
    println!("{:?}", heap);
    println!("");
    
    println!("Heap Top: {:?}", heap.pop());
    // Heap Top: Some(-4)
    println!("{:?}", heap);
    // #size: 2, [(elem: Some(-5), sub: [(elem: Some(-6), sub: [])])]
    println!("----------------------------------");

}