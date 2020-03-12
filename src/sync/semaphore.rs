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

    pub fn release(&self) {
        let mut val = self.mutex.lock().expect("failed to lock");
        val.add_assign(1);
        self.condvar.notify_all();
    }

    pub fn acquire(&self) {
        let mut val = self.mutex.lock().expect("failed to lock");
        while val.eq(&0) {
            val = self.condvar.wait(val).expect("failed to wait condvar");
        }
        val.sub_assign(1);
    }
}

unsafe impl Send for Semaphore {}

unsafe impl Sync for Semaphore {}
