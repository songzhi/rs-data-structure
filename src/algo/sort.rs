use core::{mem, ptr};

/// When dropped, copies from `src` into `dest`.
struct CopyOnDrop<T> {
    src: *mut T,
    dest: *mut T,
}

impl<T> Drop for CopyOnDrop<T> {
    fn drop(&mut self) {
        unsafe { ptr::copy_nonoverlapping(self.src, self.dest, 1); }
    }
}


/// Shifts the first element to the right until it encounters a greater or equal element.
fn shift_head<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{
    let len = v.len();
    unsafe {
        // If the first two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(1), v.get_unchecked(0)) {
            // Read the first element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let mut tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(0)));
            let mut hole = CopyOnDrop {
                src: &mut *tmp,
                dest: v.get_unchecked_mut(1),
            };
            ptr::copy_nonoverlapping(v.get_unchecked(1), v.get_unchecked_mut(0), 1);

            for i in 2..len {
                if !is_less(v.get_unchecked(i), &*tmp) {
                    break;
                }

                // Move `i`-th element one place to the left, thus shifting the hole to the right.
                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i - 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        }
    }
}

/// Shifts the last element to the left until it encounters a smaller or equal element.
fn shift_tail<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{
    let len = v.len();
    unsafe {
        // If the last two elements are out-of-order...
        if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
            // Read the last element into a stack-allocated variable. If a following comparison
            // operation panics, `hole` will get dropped and automatically write the element back
            // into the slice.
            let mut tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(len - 1)));
            let mut hole = CopyOnDrop {
                src: &mut *tmp,
                dest: v.get_unchecked_mut(len - 2),
            };
            ptr::copy_nonoverlapping(v.get_unchecked(len - 2), v.get_unchecked_mut(len - 1), 1);

            for i in (0..len - 2).rev() {
                if !is_less(&*tmp, v.get_unchecked(i)) {
                    break;
                }

                // Move `i`-th element one place to the right, thus shifting the hole to the left.
                ptr::copy_nonoverlapping(v.get_unchecked(i), v.get_unchecked_mut(i + 1), 1);
                hole.dest = v.get_unchecked_mut(i);
            }
            // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
        } else {}
    }
}

pub fn insertion_sort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool {
    for i in 1..v.len() {
        shift_tail(&mut v[..i + 1], is_less);
    }
}

pub fn shell_sort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool {
    let v_len = v.len();
    let mut increment = (v_len + 1) / 2 - 1;
    unsafe {
        while increment > 0 {
            for i in increment..v_len {
                let mut tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(i)));
                let mut j = i;
                let mut hole = CopyOnDrop {
                    src: &mut *tmp,
                    dest: v.get_unchecked_mut(j),
                };
                while j >= increment {
                    if is_less(&*tmp, &v[j - increment]) {
                        ptr::copy_nonoverlapping(v.get_unchecked(j - increment), v.get_unchecked_mut(j), 1);
                    } else {
                        break;
                    }
                    j -= increment;
                    hole.dest = v.get_unchecked_mut(j);
                }
            }
            increment = (increment + 1) / 2 - 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::algo::sort::shell_sort;

    #[test]
    fn test_shell_sort() {
        let mut v = [81, 94, 11, 96, 12, 35, 17, 95, 28, 58, 41, 75, 15];
        shell_sort(&mut v, &mut |a, b| a.lt(b));
        assert_eq!(vec![11, 12, 15, 17, 28, 35, 41, 58, 75, 81, 94, 95, 96], v);
    }
}