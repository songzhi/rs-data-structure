#![allow(clippy::mutex_atomic)]

use std::ops::{AddAssign, SubAssign};
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct Semaphore {
    mutex: Mutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    pub fn new(val: usize) -> Self {
        Self {
            mutex: Mutex::new(val),
            condvar: Condvar::new(),
        }
    }
    ///
    /// returns: the old value
    pub fn release(&self) -> usize {
        let mut val = self.mutex.lock().expect("failed to lock");
        val.add_assign(1);
        self.condvar.notify_all();
        *val - 1
    }
    ///
    /// returns: the old value
    pub fn acquire(&self) -> usize {
        let mut val = self.mutex.lock().expect("failed to lock");
        while val.eq(&0) {
            val = self.condvar.wait(val).expect("failed to wait condvar");
        }
        val.sub_assign(1);
        *val + 1
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
