use std::{cell::UnsafeCell, ops};

pub struct UnsafeBuffer<T> {
    pub capacity: usize,
    pub buffer: UnsafeCell<Vec<T>>,
    threads: usize,
}

impl<T> UnsafeBuffer<T> {
    pub fn new(capacity: usize, threads: usize) -> Self {
        UnsafeBuffer {
            capacity,
            buffer: UnsafeCell::new(Vec::<T>::with_capacity(capacity)),
            threads,
        }
    }

    pub fn write(&self, index: usize, value: T, thread_id: u8) {
        unsafe {
            let buffer = self.buffer.get().as_mut().unwrap();
            (*buffer)[index] = value;
        }
    }

    // pub fn get_valid_index(thread_id: u8, step: usize) -> 
}

impl<T> ops::Index<usize> for UnsafeBuffer<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl<T> ops::IndexMut<usize> for UnsafeBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}
