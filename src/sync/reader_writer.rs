pub mod reader_first {
    use std::cell::UnsafeCell;
    use std::ops::{Deref, DerefMut};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex, MutexGuard,
    };

    pub struct RwLock<T> {
        data: UnsafeCell<T>,
        rw_mutex: Mutex<()>,
        reader_count: AtomicUsize,
    }

    unsafe impl<T: Send> Send for RwLock<T> {}

    unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

    pub struct RwLockReadGuard<'a, T: 'a> {
        lock: &'a RwLock<T>,
        _rw_mutex_guard: Option<MutexGuard<'a, ()>>,
    }

    impl<'rwlock, T: 'rwlock> RwLockReadGuard<'rwlock, T> {
        pub fn new(
            lock: &'rwlock RwLock<T>,
            rw_mutex_guard: Option<MutexGuard<'rwlock, ()>>,
        ) -> RwLockReadGuard<'rwlock, T> {
            Self {
                lock,
                _rw_mutex_guard: rw_mutex_guard,
            }
        }
    }

    impl<T> Drop for RwLockReadGuard<'_, T> {
        fn drop(&mut self) {
            self.lock.reader_count.fetch_sub(1, Ordering::SeqCst);
        }
    }

    unsafe impl<T: Sync> Sync for RwLockReadGuard<'_, T> {}

    pub struct RwLockWriteGuard<'a, T: 'a> {
        lock: &'a RwLock<T>,
        _rw_mutex_guard: MutexGuard<'a, ()>,
    }

    unsafe impl<T: Sync> Sync for RwLockWriteGuard<'_, T> {}

    impl<'rwlock, T: 'rwlock> RwLockWriteGuard<'rwlock, T> {
        pub fn new(
            lock: &'rwlock RwLock<T>,
            rw_mutex_guard: MutexGuard<'rwlock, ()>,
        ) -> RwLockWriteGuard<'rwlock, T> {
            Self {
                lock,
                _rw_mutex_guard: rw_mutex_guard,
            }
        }
    }

    impl<T> Deref for RwLockReadGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<T> Deref for RwLockWriteGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<T> DerefMut for RwLockWriteGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe { &mut *self.lock.data.get() }
        }
    }

    impl<T> RwLock<T> {
        pub fn new(t: T) -> Self {
            Self {
                data: UnsafeCell::new(t),
                rw_mutex: Mutex::new(()),
                reader_count: AtomicUsize::new(0),
            }
        }
        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            let guard = if self.reader_count.fetch_add(1, Ordering::SeqCst) == 0 {
                self.rw_mutex.lock().ok()
            } else {
                None
            };
            RwLockReadGuard::new(self, guard)
        }
        pub fn write(&self) -> RwLockWriteGuard<'_, T> {
            let rw_mutex_guard = self.rw_mutex.lock().expect("failed to lock rw_mutex");
            RwLockWriteGuard::new(self, rw_mutex_guard)
        }
    }
}

pub mod writer_first {
    use std::cell::UnsafeCell;
    use std::ops::{Deref, DerefMut};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex, MutexGuard,
    };

    pub struct RwLock<T> {
        data: UnsafeCell<T>,
        rw_mutex: Mutex<()>,
        w_mutex: Mutex<()>,
        reader_count: AtomicUsize,
    }

    unsafe impl<T: Send> Send for RwLock<T> {}

    unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

    pub struct RwLockReadGuard<'a, T: 'a> {
        lock: &'a RwLock<T>,
        _rw_mutex_guard: Option<MutexGuard<'a, ()>>,
    }

    impl<'rwlock, T: 'rwlock> RwLockReadGuard<'rwlock, T> {
        pub fn new(
            lock: &'rwlock RwLock<T>,
            rw_mutex_guard: Option<MutexGuard<'rwlock, ()>>,
        ) -> RwLockReadGuard<'rwlock, T> {
            Self {
                lock,
                _rw_mutex_guard: rw_mutex_guard,
            }
        }
    }

    impl<T> Drop for RwLockReadGuard<'_, T> {
        fn drop(&mut self) {
            self.lock.reader_count.fetch_sub(1, Ordering::SeqCst);
        }
    }

    unsafe impl<T: Sync> Sync for RwLockReadGuard<'_, T> {}

    pub struct RwLockWriteGuard<'a, T: 'a> {
        lock: &'a RwLock<T>,
        _rw_mutex_guard: MutexGuard<'a, ()>,
        _w_mutex_guard: MutexGuard<'a, ()>,
    }

    unsafe impl<T: Sync> Sync for RwLockWriteGuard<'_, T> {}

    impl<'rwlock, T: 'rwlock> RwLockWriteGuard<'rwlock, T> {
        pub fn new(
            lock: &'rwlock RwLock<T>,
            rw_mutex_guard: MutexGuard<'rwlock, ()>,
            w_mutex_guard: MutexGuard<'rwlock, ()>,
        ) -> RwLockWriteGuard<'rwlock, T> {
            Self {
                lock,
                _rw_mutex_guard: rw_mutex_guard,
                _w_mutex_guard: w_mutex_guard,
            }
        }
    }

    impl<T> Deref for RwLockReadGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<T> Deref for RwLockWriteGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<T> DerefMut for RwLockWriteGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe { &mut *self.lock.data.get() }
        }
    }

    impl<T> RwLock<T> {
        pub fn new(t: T) -> Self {
            Self {
                data: UnsafeCell::new(t),
                rw_mutex: Mutex::new(()),
                w_mutex: Mutex::new(()),
                reader_count: AtomicUsize::new(0),
            }
        }
        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            let _w_mutex_guard = self.w_mutex.lock().expect("failed to lock w_mutex");
            let rw_mutex_guard = if self.reader_count.fetch_add(1, Ordering::SeqCst) == 0 {
                self.rw_mutex.lock().ok()
            } else {
                None
            };
            RwLockReadGuard::new(self, rw_mutex_guard)
        }
        pub fn write(&self) -> RwLockWriteGuard<'_, T> {
            let rw_mutex_guard = self.rw_mutex.lock().expect("failed to lock rw_mutex");
            let w_mutex_guard = self.w_mutex.lock().expect("failed to lock w_mutex");
            RwLockWriteGuard::new(self, rw_mutex_guard, w_mutex_guard)
        }
    }
}

pub mod limited_readers {
    use std::cell::UnsafeCell;
    use std::ops::{Deref, DerefMut};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex, MutexGuard,
    };

    use crate::sync::semaphore::Semaphore;

    pub struct RwLock<T> {
        data: UnsafeCell<T>,
        rw_mutex: Mutex<()>,
        reader_count: Semaphore,
    }

    unsafe impl<T: Send> Send for RwLock<T> {}

    unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

    pub struct RwLockReadGuard<'a, T: 'a> {
        lock: &'a RwLock<T>,
        _rw_mutex_guard: Option<MutexGuard<'a, ()>>,
    }

    impl<'rwlock, T: 'rwlock> RwLockReadGuard<'rwlock, T> {
        pub fn new(
            lock: &'rwlock RwLock<T>,
            rw_mutex_guard: Option<MutexGuard<'rwlock, ()>>,
        ) -> RwLockReadGuard<'rwlock, T> {
            Self {
                lock,
                _rw_mutex_guard: rw_mutex_guard,
            }
        }
    }

    impl<T> Drop for RwLockReadGuard<'_, T> {
        fn drop(&mut self) {
            self.lock.reader_count.release();
        }
    }

    unsafe impl<T: Sync> Sync for RwLockReadGuard<'_, T> {}

    pub struct RwLockWriteGuard<'a, T: 'a> {
        lock: &'a RwLock<T>,
        _rw_mutex_guard: MutexGuard<'a, ()>,
    }

    unsafe impl<T: Sync> Sync for RwLockWriteGuard<'_, T> {}

    impl<'rwlock, T: 'rwlock> RwLockWriteGuard<'rwlock, T> {
        pub fn new(
            lock: &'rwlock RwLock<T>,
            rw_mutex_guard: MutexGuard<'rwlock, ()>,
        ) -> RwLockWriteGuard<'rwlock, T> {
            Self {
                lock,
                _rw_mutex_guard: rw_mutex_guard,
            }
        }
    }

    impl<T> Deref for RwLockReadGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<T> Deref for RwLockWriteGuard<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            unsafe { &*self.lock.data.get() }
        }
    }

    impl<T> DerefMut for RwLockWriteGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe { &mut *self.lock.data.get() }
        }
    }

    impl<T> RwLock<T> {
        pub fn new(t: T, readers_limit: usize) -> Self {
            Self {
                data: UnsafeCell::new(t),
                rw_mutex: Mutex::new(()),
                reader_count: Semaphore::new(readers_limit),
            }
        }
        pub fn read(&self) -> RwLockReadGuard<'_, T> {
            let guard = if self.reader_count.acquire() == 0 {
                self.rw_mutex.lock().ok()
            } else {
                None
            };
            RwLockReadGuard::new(self, guard)
        }
        pub fn write(&self) -> RwLockWriteGuard<'_, T> {
            let rw_mutex_guard = self.rw_mutex.lock().expect("failed to lock rw_mutex");
            RwLockWriteGuard::new(self, rw_mutex_guard)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::AddAssign;
    use std::sync::Arc;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_reader_first() {
        use reader_first::*;
        let rwlock = Arc::new(RwLock::new(0));
        let reader0 = {
            let rwlock = rwlock.clone();
            std::thread::spawn(move || {
                let val = rwlock.read();
                assert!(val.eq(&0));
                drop(val);
                std::thread::sleep(Duration::from_millis(100));
                let val = rwlock.read();
                assert!(val.eq(&1));
            })
        };
        let writer0 = {
            let rwlock = rwlock.clone();
            std::thread::spawn(move || {
                let mut val = rwlock.write();
                val.add_assign(1);
            })
        };
        reader0.join().expect("reader0 failed");
        writer0.join().expect("writer0 failed");
    }
}
