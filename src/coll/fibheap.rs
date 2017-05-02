
//! A priority queue implemented with a fibonacci heap.
//!

// TODO: Switch out this for a more accurate blurb
//! Insertion and popping the largest element have `O(log n)` time complexity.
//! Checking the largest element is `O(1)`. Converting a vector to a binary heap
//! can be done in-place, and has `O(n)` complexity. A binary heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an `O(n
//! log n)` in-place heapsort.

#![allow(missing_docs)]
#![allow(unused_type_parameter)]

extern crate core;

// use self::core::ops::{Deref, DerefMut, Place, Placer, InPlace};
use self::core::ops::{Deref, DerefMut};
// use self::core::iter::{FromIterator, FusedIterator};
use self::core::iter::FromIterator;
use self::core::mem::{swap, size_of};
use self::core::ptr;
use self::core::fmt;

use std::slice;
use std::vec::{self, Vec};

// use std::collections::SpecExtend;

/// A priority queue implemented with a fibonacci heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the `Ord`
/// trait, changes while it is in the heap. This is normally only possible
/// through `Cell`, `RefCell`, global state, I/O, or unsafe code.
///
/// # Examples
///
/// ```
/// use std::collections::FibonacciHeap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `FibonacciHeap<i32>` in this example).
/// let mut heap = FibonacciHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("{}", x);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
pub struct FibonacciHeap<T> {
    
}

/// A container object that represents the result of the [`peek_mut()`] method
/// on `FibonacciHeap`. See its documentation for details.
///
/// [`peek_mut()`]: struct.FibonacciHeap.html#method.peek_mut
pub struct PeekMut<'a, T: 'a + Ord> {
    
}

impl<'a, T: Ord + fmt::Debug> fmt::Debug for PeekMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
    }
}

impl<'a, T: Ord> Drop for PeekMut<'a, T> {
    fn drop(&mut self) {
        
    }
}

impl<'a, T: Ord> Deref for PeekMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        
    }
}

impl<'a, T: Ord> DerefMut for PeekMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        
    }
}

impl<'a, T: Ord> PeekMut<'a, T> {
    /// Removes the peeked value from the heap and returns it.
    pub fn pop(mut this: PeekMut<'a, T>) -> T {
        
    }
}

impl<T: Clone> Clone for FibonacciHeap<T> {
    fn clone(&self) -> Self {
        
    }

    fn clone_from(&mut self, source: &Self) {
        
    }
}

impl<T: Ord> Default for FibonacciHeap<T> {
    /// Creates an empty `FibonacciHeap<T>`.
    #[inline]
    fn default() -> FibonacciHeap<T> {

    }
}

impl<T: fmt::Debug + Ord> fmt::Debug for FibonacciHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
    }
}

impl<T: Ord> FibonacciHeap<T> {
    /// Creates an empty `FibonacciHeap` as a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    /// heap.push(4);
    /// ```
    pub fn new() -> FibonacciHeap<T> {
        
    }

    /// Creates an empty `FibonacciHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `FibonacciHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::with_capacity(10);
    /// heap.push(4);
    /// ```
    pub fn with_capacity(capacity: usize) -> FibonacciHeap<T> {
        
    }

    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let heap = FibonacciHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter() {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<T> {
        
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    pub fn peek(&self) -> Option<&T> {
        
    }

    /// Returns a mutable reference to the greatest item in the binary heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    pub fn peek_mut(&mut self) -> Option<PeekMut<T>> {
        
    }

    /// Returns the number of elements the binary heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::with_capacity(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn capacity(&self) -> usize {
        
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `FibonacciHeap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer `reserve` if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    /// heap.reserve_exact(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `FibonacciHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap: FibonacciHeap<i32> = FibonacciHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity() == 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        
    }

    /// Pushes an item onto the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 3);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    pub fn push(&mut self, item: T) {
        
    }


    /// Consumes the `FibonacciHeap` and returns the underlying vector
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let heap = FibonacciHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    /// let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{}", x);
    /// }
    /// ```
    pub fn into_vec(self) -> Vec<T> {
        
    }

    /// Consumes the `FibonacciHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    ///
    /// let mut heap = FibonacciHeap::from(vec![1, 2, 4, 5, 7]);
    /// heap.push(6);
    /// heap.push(3);
    ///
    /// let vec = heap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        
    }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.
    // fn sift_up(&mut self, start: usize, pos: usize) -> usize { }
    // fn sift_down_range(&mut self, pos: usize, end: usize) { }
    // fn sift_down(&mut self, pos: usize) { }
    // fn sift_down_to_bottom(&mut self, mut pos: usize) { }

    /// Returns the length of the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let heap = FibonacciHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        
    }

    /// Checks if the binary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        
    }

    /// Clears the binary heap, returning an iterator over the removed elements.
    ///
    /// The elements are removed in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    pub fn drain(&mut self) -> Drain<T> {

    }

    /// Drops all items from the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let mut heap = FibonacciHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    ///
    /// let v = vec![-10, 1, 2, 3, 3];
    /// let mut a = FibonacciHeap::from(v);
    ///
    /// let v = vec![-20, 5, 43];
    /// let mut b = FibonacciHeap::from(v);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    pub fn append(&mut self, other: &mut Self) {

    }
}

/// Hole represents a hole in a slice i.e. an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    
}

impl<'a, T> Hole<'a, T> {
    /// Create a new Hole at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        
    }

    #[inline]
    fn pos(&self) -> usize {
        
    }

    /// Return a reference to the element removed
    #[inline]
    fn element(&self) -> &T {
        
    }

    /// Return a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, index: usize) {
        
    }
}

impl<'a, T> Drop for Hole<'a, T> {
    #[inline]
    fn drop(&mut self) {
        
    }
}

/// `FibonacciHeap` iterator.
pub struct Iter<'a, T: 'a> {
    
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
    }
}

// FIXME(#19839) Remove in favor of `#[derive(Clone)]`
impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Iter<'a, T> {
        
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        
    }
}


// impl<'a, T> ExactSizeIterator for Iter<'a, T> {
//     fn is_empty(&self) -> bool {
        
//     }
// }

// impl<'a, T> FusedIterator for Iter<'a, T> {}

/// An iterator that moves out of a `FibonacciHeap`.
#[derive(Clone)]
pub struct IntoIter<T> {
    
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        
    }
}

// impl<T> ExactSizeIterator for IntoIter<T> {
//     fn is_empty(&self) -> bool {
        
//     }
// }

// impl<T> FusedIterator for IntoIter<T> {}

/// An iterator that drains a `FibonacciHeap`.
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    
}

impl<'a, T: 'a> Iterator for Drain<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        
    }
}

impl<'a, T: 'a> DoubleEndedIterator for Drain<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        
    }
}

// impl<'a, T: 'a> ExactSizeIterator for Drain<'a, T> {
//     fn is_empty(&self) -> bool {
        
//     }
// }

// impl<'a, T: 'a> FusedIterator for Drain<'a, T> {}

impl<T: Ord> From<Vec<T>> for FibonacciHeap<T> {
    fn from(vec: Vec<T>) -> FibonacciHeap<T> {
        
    }
}

// Why do I need to do this ???
// impl<T> From<FibonacciHeap<T>> for Vec<T> {
//     fn from(heap: FibonacciHeap<T>) -> Vec<T> {
        
//     }
// }

impl<T: Ord> FromIterator<T> for FibonacciHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> FibonacciHeap<T> {
        
    }
}

impl<T: Ord> IntoIterator for FibonacciHeap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the binary heap in arbitrary order. The binary heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use std::collections::FibonacciHeap;
    /// let heap = FibonacciHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{}", x);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        
    }
}

impl<'a, T: Ord> IntoIterator for &'a FibonacciHeap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        
    }
}

impl<T: Ord> Extend<T> for FibonacciHeap<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        
    }
}

// impl<T: Ord, I: IntoIterator<Item = T>> SpecExtend<I> for FibonacciHeap<T> {
//     default fn spec_extend(&mut self, iter: I) {
        
//     }
// }

// impl<T: Ord> SpecExtend<FibonacciHeap<T>> for FibonacciHeap<T> {
//     fn spec_extend(&mut self, ref mut other: FibonacciHeap<T>) {
        
//     }
// }

impl<T: Ord> FibonacciHeap<T> {
    fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for FibonacciHeap<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        
    }
}

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// pub struct FibonacciHeapPlace<'a, T: 'a + Clone + Ord> {
//     
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: Clone + Ord + fmt::Debug> fmt::Debug for FibonacciHeapPlace<'a, T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: 'a + Clone + Ord> Placer<T> for &'a mut FibonacciHeap<T> {
//     type Place = FibonacciHeapPlace<'a, T>;

//     fn make_place(self) -> Self::Place {
//         
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: Clone + Ord> Place<T> for FibonacciHeapPlace<'a, T> {
//     fn pointer(&mut self) -> *mut T {
//
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: Clone + Ord> InPlace<T> for FibonacciHeapPlace<'a, T> {
//     type Owner = &'a T;

//     unsafe fn finalize(self) -> &'a T {
//         
//     }
// }