use owned_alloc::OwnedAlloc;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct Node<T> {
    elem: ManuallyDrop<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node {
            elem: ManuallyDrop::new(elem),
            next: AtomicPtr::default(),
        }
    }
}

type Link<T> = AtomicPtr<Node<T>>;

#[derive(Default)]
pub struct List<T> {
    head: Link<T>,
}

unsafe impl<T> Send for List<T> where T: Send {}

unsafe impl<T> Sync for List<T> where T: Send {}

impl<T> List<T> {
    pub fn push(&self, elem: T) {
        let node = OwnedAlloc::new(Node::new(elem)).into_raw();
        let mut head = self.head.load(Ordering::Relaxed);
        loop {
            unsafe {
                node.as_ref().next.store(head, Ordering::Relaxed);
            }
            match self.head.compare_exchange(
                head,
                node.as_ptr(),
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    break;
                }
                Err(ptr) => head = ptr,
            }
        }
    }
    pub fn pop(&self) -> Option<T> {
        unsafe {
            let mut head = NonNull::new(self.head.load(Ordering::Relaxed))?;
            loop {
                match self.head.compare_exchange(
                    head.as_ptr(),
                    head.as_ref().next.load(Ordering::SeqCst),
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        head.as_ptr().drop_in_place();
                        break Some((&mut *head.as_mut().elem as *mut T).read());
                    }
                    Err(ptr) => head = NonNull::new(ptr)?,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn no_data_corruption() {
        const NTHREAD: usize = 20;
        const NITER: usize = 500;
        const NMOD: usize = 55;

        let stack = Arc::new(List::default());
        let mut handles = Vec::with_capacity(NTHREAD);

        for i in 0..NTHREAD {
            let stack = stack.clone();
            handles.push(thread::spawn(move || {
                for j in 0..NITER {
                    let val = (i * NITER) + j;
                    stack.push(val);
                    if (val + 1) % NMOD == 0 {
                        if let Some(val) = stack.pop() {
                            assert!(val < NITER * NTHREAD);
                        }
                    }
                }
            }));
        }

        for handle in handles {
            handle.join().expect("thread failed");
        }

        let expected = NITER * NTHREAD - NITER * NTHREAD / NMOD;
        let mut res = 0;
        while let Some(val) = stack.pop() {
            assert!(val < NITER * NTHREAD);
            res += 1;
        }

        assert_eq!(res, expected);
    }
}
