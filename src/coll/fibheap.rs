use std::cmp::{Ordering, max};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

// TODO: Refactoring
// TODO: Figure out dec_key operation

/*
It's odd how easy it is to come back to Rust, how Rust actually feels nice and all. I'm not sure why though.

I remember I gave up on using Rust to implement Spero because I kept running into the borrow checker, but
I didn't really have any issue with it here, outside of a few areas where I had to tiptoe around it.

But overall it just went really nice and easy, extremely unintrusive. Is this because of all my work on Spero,
which models a lot of its design on Rust, or on the compiler, particularly my insistence on unique_ptr which
mirrors a lot of Rust's "borrow-checking" semantics.

 */

pub struct FibHeap<T> {
    forest: Vec<FibHeapNode<T>>,
    max_index: usize,
    count: usize
}

impl<T: Ord + Debug> FibHeap<T> {
    pub fn new() -> FibHeap<T> {
        FibHeap{
            forest: Vec::with_capacity(10),
            max_index: 0,
            count: 0,
        }
    }

    // Note: In an effort to mimic the interface provided in the rust standard library
    // DecKey is not a supported operation

    // Add an element to the heap
    pub fn push(&mut self, elem: T) {
        // Update `max_index` if the new element is larger than the old max
        if FibHeap::compare_elements(Some(&elem), self.peek()) == Ordering::Greater {
            self.max_index = self.forest.len();
        }

        // Add the new element to `forest`
        self.count += 1;
        self.forest.push(FibHeapNode{ elem: Some(elem), subnodes: vec!(), marked: false, rank: 0 });
    }

    // Get the max element if one exists
    pub fn peek(&self) -> Option<&T> {
        self.forest.get(self.max_index).and_then(|tree| tree.elem.as_ref())
    }

    // pub fn peek_mut(&mut self) -> Option<PeekMut<T>> {
    //     if self.is_empty() {
    //         None
    //     } else {
    //         Some(PeekMut{ heap: self, sift: true, item: None })
    //     }
    // }
    
    // Move all elements from another heap into this one
    pub fn merge(&mut self, mut other: Self) {
        self.append(&mut other);
    }

    // This might have the same behavior as `merge`
    pub fn append(&mut self, other: &mut FibHeap<T>) {
        if FibHeap::compare_elements(self.peek(), other.peek()) == Ordering::Less {
            self.max_index = self.forest.len() + other.max_index;
        }

        self.forest.append(&mut other.forest);
        self.count += other.count;
    }

    // Clears the heap
    pub fn clear(&mut self) {
        self.forest.clear()
    }

    // Puts the heap into the fibonacci state after a pop
    fn restore_state(&mut self) {
        let mut i = 0;
        let mut rank_map = HashMap::new();              // Map[NodeRank => NodeIndex]

        loop {
            // Due to borrowing of `rank_map` and `forest` in the initial match statement
            // `foo` gets assigned to a StateBehavior enum in order to control the updating
            // of `rank_map` (reflect heap condensing) and the exiting of the parent loop
            let foo = match self.forest.get(i).and_then(|tree| Some(rank_map.entry(tree.rank))) {
                Some(Entry::Occupied(former)) => {
                    let (_, idx) = former.remove_entry();

                    // Order the nodes so that the biggest element remains in the vector
                    match FibHeap::compare_nodes(&self.forest[i], &self.forest[idx]) {
                        Ordering::Greater => self.forest.swap(i, idx),
                        _ => (),
                    }

                    // Get the two trees to operate on
                    let cur_tree = self.forest.remove(i);
                    let ref mut parent = self.forest[idx];

                    // Merge the old tree into the new parent
                    parent.rank = max(parent.rank, cur_tree.rank + 1);
                    parent.subnodes.push(cur_tree);

                    // Specify the new rank assignment
                    StateBehavior::Modify(parent.rank, idx)
                },
                Some(Entry::Vacant(entry)) => {
                    // Modify the entry inplace and move on
                    entry.insert(i);
                    StateBehavior::Continue
                },
                None => StateBehavior::Break,
            };

            // Handle the StateBehavior enum as needed
            match foo {
                StateBehavior::Modify(rank, idx) => { rank_map.insert(rank, idx); },
                StateBehavior::Break => break,
                StateBehavior::Continue => { i += 1; }
            }
        }
    }

    // Pop the top element from the heap
    pub fn pop(&mut self) -> Option<T> {
        // Put the max element on the end of the stack (`pop` apparently doesn't have borrowing issues)
        let back_index = self.forest.len();
        if back_index > 0 && back_index - 1 != self.max_index {
            self.forest.swap(self.max_index, back_index - 1);
        }

        // Get the max element if possible
        match self.forest.pop() {
            // The tree was empty
            None => None,

            // Fix up the heap
            Some(mut tree) => {
                self.count -= 1;

                // Add the popped subtree to the main forest
                self.forest.append(&mut tree.subnodes);
                
                // Restore the fibonacci state
                self.restore_state();

                // Find the max element in the new heap and store its index
                self.max_index =
                    match self.forest.iter().enumerate().max_by(|a, b| FibHeap::compare_nodes(a.1, b.1)) {
                        Some((idx, _)) => idx,
                        None => 0,
                    };

                // Return the element
                tree.elem
            },
        }
    }

    // Get the number of elements stored in the heap
    pub fn len(&self) -> usize {
        self.count
    }

    // Check if the heap is empty (ie. len == 0)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    // Helper methods to abstract the process of ordering FibHeapNodes (could just implement Ordering)
    fn compare_nodes(a: &FibHeapNode<T>, b: &FibHeapNode<T>) -> Ordering {
        FibHeap::compare_elements(a.elem.as_ref(), b.elem.as_ref())
    }

    fn compare_elements(a: Option<&T>, b: Option<&T>) -> Ordering {
        match (a, b) {
            (Some(a), Some(b)) if a < b => Ordering::Less,
            (Some(a), Some(b)) if a > b => Ordering::Greater,
            (Some(_), _) => Ordering::Less,
            (_, Some(_)) => Ordering::Greater,
            _ => Ordering::Equal
        }
    }

}

impl<T: Ord + Debug> Debug for FibHeapNode<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "(elem: {:?}, sub: {:?})", self.elem, self.subnodes)
    }
}

impl<T: Ord + Debug> Debug for FibHeap<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#size: {}, {:?}", self.count, self.forest)
    }
}

// Implementation structure for the FibHeap tree
#[allow(dead_code)]
struct FibHeapNode<T> {
    elem: Option<T>,
    subnodes: Vec<FibHeapNode<T>>,
    marked: bool,
    rank: usize,
}

// Structure wrapping a mutable reference on the "largest" element in a fib heap
// pub struct PeekMut<'a, T: 'a + Ord + Debug> {
//     heap: &'a mut FibHeap<T>,
//     item: Option<T>,
//     sift: bool,
// }


// impl<'a, T: Ord + Debug> Debug for PeekMut<'a, T> {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
//         f.debug_tuple("PeekMut")
//          .field(&self.heap.forest[0])
//          .finish()
//     }
// }

// impl<'a, T: Ord + Debug> Drop for PeekMut<'a, T> {
//     fn drop(&mut self) {
//         if self.sift {
//             self.heap.pop();
//         }
//     }
// }

// impl<'a, T: Ord + Debug> ops::Deref for PeekMut<'a, T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         self.item = self.heap.forest[0].elem;
//         &self.item.unwrap()
//     }
// }

// impl<'a, T: Ord + Debug> ops::DerefMut for PeekMut<'a, T> {
//     fn deref_mut(&mut self) -> &mut T {
//         &mut self.heap.forest[0].elem.unwrap()
//     }
// }

// impl<'a, T: Ord + Debug> PeekMut<'a, T> {
//     /// Removes the peeked value from the heap and returns it.
//     pub fn pop(mut this: PeekMut<'a, T>) -> T {
//         let value = this.heap.pop().unwrap();
//         value
//     }
// }

enum StateBehavior {
    Break,
    Continue,
    Modify(usize, usize),
}