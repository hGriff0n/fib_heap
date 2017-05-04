

struct FibHeapNode<T: Ord> {
    elem: Option<T>,
}

pub struct FibHeap<T: Ord> {
    trees: Vec<FibHeapNode<T>>,
    max_index: usize,
    count: usize
}

impl<T: Ord> FibHeap<T> {
    pub fn new() -> FibHeap<T> {
        FibHeap{
            trees: vec!(),
            max_index: 0,
            count: 0,
        }
    }

    // Add an element to the heap
    pub fn push(&mut self, elem: T) {
        self.count += 1;

        // Why can't I write "if let Some(e) = self.peek() {" ??? (assignment to borrowed 'self.max_index')
        if let Some(e) = self.trees.get(self.max_index).and_then(|tree| tree.elem.as_ref()) {
            if e < &elem {
                self.max_index = self.trees.len();
            }
        }
        
        self.trees.push(FibHeapNode{ elem: Some(elem) });
    }

    // Get the max element if one exists
    pub fn peek(&self) -> Option<&T> {
        self.trees.get(self.max_index).and_then(|tree| tree.elem.as_ref())
    }
    // Move all elements from another heap into this one
    pub fn merge(&mut self, other: Self) {
        
    }

    // Pop the top element from the heap
    pub fn pop(&mut self) -> Option<T> {
        match self.trees.get(self.max_index) {
            Some(tree) => {
                None
            },
            None => None,
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

    // functions to look at adding later
    // append - merge but doesn't destroy the other heap
    // clear - remove all elements from the heap
    // drain - cleans the heap, returns an iterator over the removed elements
    // into_sorted_vec
    // into_vec
    // peek_mut
    // iter
}