/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) where T: PartialOrd {
        //TODO
        if self.count <= 0 {
            self.items[0] = value;
            self.count += 1;
            return;
        }
        self.items.push(value);
        self.count += 1;
        let mut current_index = self.items.len() - 1;
        while current_index > 0 {
            let pindex = self.parent_idx(current_index);
            let pnode = &self.items[pindex];
            let current_node = &self.items[current_index];
            if !(self.comparator)(pnode, current_node) {
                self.items.swap(pindex, current_index);
                current_index = pindex;
            } else {
                break;
            }
            
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) < self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {

        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right < self.count {
            if (self.comparator)(&self.items[right], &self.items[left]) {
                right
            } else {
                left
            }
        } else {
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<T>{
        //TODO
		if self.count == 0 {
            return None;
        }
        let last_idx = self.count - 1;
        self.items.swap(0, last_idx);
        let result = self.items.pop().unwrap();
        self.count -= 1;

        let mut current_index = 0;
        while self.children_present(current_index) {
            let child_idx = self.smallest_child_idx(current_index);
            if (self.comparator)(&self.items[child_idx], &self.items[current_index]) {
                self.items.swap(current_index, child_idx);
                current_index = child_idx;
            } else {
                break;
            }

        }
        
        return Some(result);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}