/*
    properties:
        * ptr: points to the first location of the actual data on the heap, data is contiguous
        * len: number of elements in the data
        * cap: capacity of the array data
*/

use std::{
    alloc::{Layout, handle_alloc_error},
    ops::{Index, IndexMut},
    ptr::NonNull,
};

pub struct CustomVector {
    cap: usize,
    len: usize,
    ptr: NonNull<i32>,
}

impl CustomVector {
    pub fn new() -> Self {
        Self {
            cap: 0,
            len: 0,
            ptr: NonNull::dangling(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        // 0 byte allocation may lead to errors, so check if input capacity is zero
        if capacity == 0 {
            return Self::new();
        }

        // newly created vecor, but capacity is defined at creation.
        // so, define a layout and allocate memory, then assign the pointer
        let layout = std::alloc::Layout::array::<i32>(capacity).expect("");
        let alloc_ptr = unsafe { std::alloc::alloc(layout) };

        let data_ptr = match NonNull::new(alloc_ptr as *mut i32) {
            Some(ptr) => ptr,
            None => handle_alloc_error(layout),
        };

        Self {
            cap: capacity,
            len: 0,
            ptr: data_ptr,
        }
    }

    pub fn first(&self) -> Option<&i32> {
        if self.len == 0 {
            // no element in data array, return None
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr()) }
        }
    }

    pub fn last(&self) -> Option<&i32> {
        if self.len == 0 {
            // no element in data array, return None
            None
        } else {
            // ptr pointing to first element, to get the last element we need to
            // move pointer self.len - 1
            // assume array is: 0,1,2 then self.len is 3
            // if we move 3 times, we are trying to point something out of bounds of the array
            // we need to advance the pointer self.len - 1, so we reach the last elements index
            unsafe { Some(&*self.ptr.as_ptr().add(self.len - 1)) }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(self.ptr.as_ptr().add(self.len).read()) }
        }
    }

    pub fn get(&self, index: usize) -> Option<&i32> {
        if self.len <= index {
            return None;
        }

        unsafe { Some(&*self.ptr.as_ptr().add(index)) }
    }

    pub fn push_back(&mut self, data: i32) {
        if self.capacity_is_full() {
            self.grow();
        }

        unsafe {
            // advance pointer len * size_of_t bytes
            self.ptr.as_ptr().add(self.len).write(data);
        }

        self.len += 1;
    }

    fn capacity_is_full(&self) -> bool {
        self.cap == self.len
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let mem_layout = std::alloc::Layout::array::<i32>(new_cap).expect("");

        let new_ptr = if self.cap == 0 {
            // vector is newly created and needs contigous array
            // to do allocations, we need a memory layout.
            // alloc, allocates memory on ram, we need to deallocate old allocations when needed (grow and drop)
            // allocation with 0 size may lead to an undefined behaviour, hence starting capacity 4
            unsafe { std::alloc::alloc(mem_layout) }
        } else {
            // vector has array data, new array must be allocated and the old pointer should point
            // new one. Old data must be deleted after copied into new array.
            let old_layout = std::alloc::Layout::array::<i32>(self.cap).expect("");
            unsafe {
                // reallocate frees old layout when necessary
                // if new allocation available contiguous to previous one, then old data is not removed, only allocation is grew
                // if new allocation is not possible contigouous to old allocation, then old allocation is freed and old data copied to new allocation
                std::alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, mem_layout.size())
            }
        };

        self.ptr = match NonNull::new(new_ptr as *mut i32) {
            Some(p) => p,
            None => handle_alloc_error(mem_layout),
        };

        self.cap = new_cap;
    }

    pub fn get_len(&self) -> usize {
        self.len
    }

    pub fn get_capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Drop for CustomVector {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = std::alloc::Layout::array::<i32>(self.cap).unwrap();
            unsafe {
                // free the memory that self.ptr points to
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl Index<usize> for CustomVector {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Index out of bounds.");
        unsafe { &*self.ptr.as_ptr().add(index) }
    }
}

impl IndexMut<usize> for CustomVector {
    // here, the Output type is known because of Index trait.
    // Index trait is supertrait of IndexMut
    // so Index must be implemented before IndexMut
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len, "Index out of bounds.");
        unsafe { &mut *self.ptr.as_ptr().add(index) }
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut vec = CustomVector::new();

        vec.push_back(24);
        vec.push_back(29);
        vec.push_back(31);
        vec.push_back(65);
        vec.push_back(22);
        vec.push_back(-1223);
        vec.push_back(23432);

        assert_eq!(vec.get_len(), 7);

        assert_eq!(vec.pop(), Some(23432));
        assert_eq!(vec.pop(), Some(-1223));
        assert_eq!(vec.pop(), Some(22));
        assert_eq!(vec.pop(), Some(65));

        assert_eq!(vec.get_len(), 3);

        vec.push_back(234322);
        assert_eq!(vec.get_len(), 4);
    }

    #[test]
    fn indexable() {
        let mut vec = CustomVector::new();

        vec.push_back(24);
        vec.push_back(25);
        vec.push_back(26);
        vec.push_back(62);

        assert_eq!(vec[0], 24);
        assert_eq!(vec[1], 25);
        assert_eq!(vec.pop(), Some(62));
        assert_eq!(vec.pop(), Some(26));
        assert_eq!(vec.pop(), Some(25));
        assert_eq!(vec[0], 24);
    }

    #[test]
    #[should_panic]
    fn idx_oob() {
        let mut vec = CustomVector::new();

        vec.push_back(24);
        vec.push_back(25);
        vec.push_back(26);
        // this line should panic
        assert_eq!(vec[vec.get_len()], 25);
    }

    #[test]
    fn creates_with_capacity() {
        let cap = 24;
        let mut vec = CustomVector::with_capacity(cap);

        assert_eq!(cap, vec.get_capacity());
        assert_eq!(0, vec.get_len());

        for i in 0..cap {
            vec.push_back((i + 1) as i32);
        }

        assert_eq!(24, vec.get_len());
        assert_eq!(24, vec.get_capacity());
        // vector will grow here, from 24 to 48
        vec.push_back(554);

        assert_eq!(25, vec.get_len());
        assert_eq!(48, vec.get_capacity());
    }

    #[test]
    fn returns_first() {
        let mut vec = CustomVector::with_capacity(22);

        assert_eq!(vec.first(), None);

        vec.push_back(24);
        vec.push_back(44);
        vec.push_back(33);

        assert_eq!(vec.first(), Some(&24));
        vec.pop();
        vec.pop();
        assert_eq!(vec.first(), Some(&24));
        vec.pop();

        vec.push_back(64);
        assert_eq!(vec.first(), Some(&64));
    }

    #[test]
    fn returns_last() {
        let mut vec = CustomVector::with_capacity(48);

        assert_eq!(vec.last(), None);

        vec.push_back(24);
        vec.push_back(44);
        vec.push_back(33);

        assert_eq!(vec.last(), Some(&33));
        vec.pop();
        vec.pop();
        assert_eq!(vec.last(), Some(&24));
        vec.pop();

        vec.push_back(64);
        assert_eq!(vec.last(), Some(&64));
    }
}
