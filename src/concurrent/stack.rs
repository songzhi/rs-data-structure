//! [Reference](http://moodycamel.com/blog/2014/solving-the-aba-problem-for-lock-free-free-lists)

use std::sync::atomic::{AtomicU32, AtomicPtr, Ordering};
use std::mem::ManuallyDrop;
use owned_alloc::OwnedAlloc;
use std::ptr::{null_mut, NonNull};


pub struct Node<T> {
    val: ManuallyDrop<T>,
    refs: AtomicU32,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(val: T, next: *mut Node<T>) -> Self {
        Node {
            val: ManuallyDrop::new(val),
            refs: AtomicU32::new(0),
            next: AtomicPtr::new(next),
        }
    }
}

const REFS_MASK: u32 = 0x7fff_ffff;
const SHOULD_BE_ON_STACK: u32 = 0x8000_0000;


pub struct Stack<T> {
    top: AtomicPtr<Node<T>>,

}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack {
            top: AtomicPtr::default()
        }
    }
}

unsafe impl<T> Send for Stack<T> where T: Send {}

unsafe impl<T> Sync for Stack<T> where T: Send {}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&self, val: T) {
        unsafe {
            let top = OwnedAlloc::new(Node::new(val, null_mut()));
            self.add_knowing_refs_is_0(top.into_raw().as_ptr());
        }
    }
    pub fn pop(&self) -> Option<T> {
        let mut head = NonNull::new(self.top.load(Ordering::Acquire))?;
        unsafe {
            loop {
                let refs = head.as_ref().refs.load(Ordering::Relaxed);
                if (refs & REFS_MASK) == 0 || head.as_ref().refs.compare_exchange(refs, refs + 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
                    head = NonNull::new(self.top.load(Ordering::Acquire))?;
                    continue;
                }

                // Good, reference count has been incremented (it wasn't at zero), which means
                // we can read the next and not worry about it changing between now and the time
                // we do the CAS
                let next = head.as_ref().next.load(Ordering::Relaxed);
                match self.top.compare_exchange(head.as_ptr(), next, Ordering::Acquire, Ordering::Relaxed) {
                    Ok(_) => {
                        // Yay, got the node. This means it was on the list, which means
                        // shouldBeOnFreeList must be false no matter the refcount (because
                        // nobody else knows it's been taken off yet, it can't have been put back on).
                        assert_eq!(head.as_ref().refs.load(Ordering::Relaxed) & SHOULD_BE_ON_STACK, 0);
                        // Decrease refcount twice, once for our ref, and once for the list's ref
                        head.as_ref().refs.fetch_sub(2, Ordering::Relaxed);
                        head.as_ptr().drop_in_place();
                        break Some((&mut *head.as_mut().val as *mut T).read());
                    }
                    Err(ptr) => {
                        // OK, the head must have changed on us, but we still need to decrease the refcount we
                        // increased
                        let refs = head.as_ref().refs.fetch_sub(1, Ordering::AcqRel);
                        if refs == SHOULD_BE_ON_STACK + 1 {
                            self.add_knowing_refs_is_0(head.as_ptr());
                        }
                        head = NonNull::new(ptr)?;
                    }
                };
            }
        }
    }
    /// Since the refcount is zero, and nobody can increase it once it's zero (except us, and we
    /// run only one copy of this method per node at a time, i.e. the single thread case), then we
    /// know we can safely change the next pointer of the node; however, once the refcount is back
    /// above zero, then other threads could increase it (happens under heavy contention, when the
    /// refcount goes to zero in between a load and a refcount increment of a node in try_get, then
    /// back up to something non-zero, then the refcount increment is done by the other thread) --
    /// so, if the CAS to add the node to the actual list fails, decrease the refcount and leave
    /// the add operation to the next thread who puts the refcount back at zero (which could be us,
    /// hence the loop).
    unsafe fn add_knowing_refs_is_0(&self, node: *mut Node<T>) {
        let mut head = self.top.load(Ordering::Relaxed);
        loop {
            (*node).next.store(head, Ordering::Relaxed);
            (*node).refs.store(1, Ordering::Release);
            match self.top.compare_exchange(head, node, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => {}
                Err(ptr) => {
                    head = ptr;
                    if (*node).refs.fetch_add(SHOULD_BE_ON_STACK - 1, Ordering::Release) == 1 {
                        continue;
                    }
                }
            };
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn on_empty_first_pop_is_none() {
        let stack = Stack::<usize>::new();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn on_empty_last_pop_is_none() {
        let stack = Stack::new();
        stack.push(3);
        stack.push(1234);
        stack.pop();
        stack.pop();
        assert!(stack.pop().is_none());
    }

    #[test]
    fn order() {
        let stack = Stack::new();
        stack.push(4);
        stack.push(3);
        stack.push(5);
        stack.push(6);
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(3));
    }

    #[test]
    fn no_data_corruption() {
        const NTHREAD: usize = 20;
        const NITER: usize = 800;
        const NMOD: usize = 55;

        let stack = Arc::new(Stack::new());
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
