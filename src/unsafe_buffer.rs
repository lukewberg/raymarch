use std::{cell::UnsafeCell, ops};

pub struct UnsafeBuffer<T> {
    pub capacity: usize,
    pub buffer: UnsafeCell<Vec<T>>,
    pub threads: usize,
}

unsafe impl<T> Sync for UnsafeBuffer<T> {}

impl<T> UnsafeBuffer<T> {
    pub fn new(capacity: usize, threads: usize) -> Self {
        let mut buffer = UnsafeCell::new(Vec::<T>::with_capacity(capacity));
        unsafe {
            buffer.get_mut().set_len(capacity);
        }
        UnsafeBuffer {
            capacity,
            buffer,
            threads,
        }
    }

    pub fn write(&self, index: usize, value: T, thread_id: usize) {
        unsafe {
            let buffer = self.buffer.get().as_mut().unwrap();
            (*buffer)[index] = value;
        }
    }

    pub fn get_valid_index(&self, thread_id: usize, step: usize) -> Option<usize> {
        if thread_id <= self.threads{
            let result = ((self.threads - 1) * step) + thread_id;
            if result < self.capacity {
                return Some(result);
            }
            // match thread_id {
            //     0 => {
            //         return Some(step * (self.threads - 1));
            //     }
            //     _ => {
            //         return Some((step * (self.threads)) + thread_id);
            //     }
            // }
        }
        None
    }
}

impl<T> ops::Index<usize> for UnsafeBuffer<T>
// where
//     for<'a> T: 'a,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &((self
                .buffer
                .get()
                .as_ref()
                .expect("Failed to read from shared UnsafeBuffer, abort!"))[index])
        }
    }
}

impl<T> ops::IndexMut<usize> for UnsafeBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        todo!()
    }
}
