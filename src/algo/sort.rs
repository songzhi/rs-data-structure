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
        }
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
    let mut increment = v_len / 2;
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
                        // hole.dest = v.get_unchecked_mut(j);
                        break;
                    }
                    j -= increment;
                    hole.dest = v.get_unchecked_mut(j);
                }
            }
            increment = (increment - 1) / 3;
        }
    }
}

pub fn heapsort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool
{
    // This binary heap respects the invariant `parent >= child`.
    let mut sift_down = |v: &mut [T], mut node| {
        loop {
            // Children of `node`:
            let left = 2 * node + 1;
            let right = 2 * node + 2;

            // Choose the greater child.
            let greater = if right < v.len() && is_less(&v[left], &v[right]) {
                right
            } else {
                left
            };

            // Stop if the invariant holds at `node`.
            if greater >= v.len() || !is_less(&v[node], &v[greater]) {
                break;
            }

            // Swap `node` with the greater child, move one step down, and continue sifting.
            v.swap(node, greater);
            node = greater;
        }
    };

    // Build the heap in linear time.
    for i in (0..v.len() / 2).rev() {
        sift_down(v, i);
    }

    // Pop maximal elements from the heap.
    for i in (1..v.len()).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0);
    }
}

pub fn merge_sort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool {
    let v_len = v.len();
    let mut tmp = Vec::with_capacity(v_len);
    sort(v, &mut tmp, 0, v_len - 1, is_less);

    fn sort<T, F>(v: &mut [T], tmp: &mut [T], left: usize, right: usize, is_less: &mut F)
        where F: FnMut(&T, &T) -> bool {
        if left < right {
            let center = (left + right) / 2;
            sort(v, tmp, left, center, is_less);
            sort(v, tmp, center + 1, right, is_less);
            merge(v, tmp, left, center + 1, right, is_less);
        }
    }

    fn merge<T, F>(v: &mut [T], tmp: &mut [T], left: usize, right: usize, right_end: usize, is_less: &mut F)
        where F: FnMut(&T, &T) -> bool {
        let mut left = left;
        let mut right = right;
        let left_end = right - 1;
        let mut tmp_pos = left;
        let num_elems = right_end - left + 1;

        unsafe {
            while left <= left_end && right <= right_end {
                if is_less(&v[left], &v[right]) {
                    ptr::copy_nonoverlapping(v.get_unchecked(left), tmp.get_unchecked_mut(tmp_pos), 1);
                    left += 1;
                } else {
                    ptr::copy_nonoverlapping(v.get_unchecked(right), tmp.get_unchecked_mut(tmp_pos), 1);
                    right += 1;
                }
                tmp_pos += 1;
            }
            if left <= left_end {
                let left_half_rest = left_end - left + 1;
                ptr::copy_nonoverlapping(v.get_unchecked(left), tmp.get_unchecked_mut(tmp_pos), left_half_rest);
                tmp_pos += left_half_rest;
            }
            if right <= right_end {
                let right_half_rest = right_end - right + 1;
                ptr::copy_nonoverlapping(v.get_unchecked(right), tmp.get_unchecked_mut(tmp_pos), right_half_rest);
            }
            let old_left = right_end + 1 - num_elems;
            ptr::copy_nonoverlapping(tmp.get_unchecked(old_left), v.get_unchecked_mut(old_left), num_elems);
        }
    }
}

pub fn quick_sort<T, F>(v: &mut [T], is_less: &mut F)
    where F: FnMut(&T, &T) -> bool {
    sort(v, 0, v.len() - 1, is_less);

    fn sort<T, F>(v: &mut [T], left: usize, right: usize, is_less: &mut F)
        where F: FnMut(&T, &T) -> bool {
        const CUTOFF: usize = 3;
        if left + CUTOFF <= right {
            return insertion_sort(v.split_at_mut(left).1, is_less);
        }
        let (pivot, _) = choose_pivot(v, is_less);
        let mut i = left;
        let mut j = right - 1;
        unsafe {
            loop {
                while is_less(v.get_unchecked(i + 1), v.get_unchecked(pivot)) { i += 1; }
                while !is_less(v.get_unchecked(j - 1), v.get_unchecked(pivot)) { j -= 1; }
                if i < j {
                    ptr::swap(v.get_unchecked_mut(i), v.get_unchecked_mut(j));
                } else {
                    break;
                }
            }
            ptr::swap(v.get_unchecked_mut(i), v.get_unchecked_mut(right - 1));
            sort(v, left, i - 1, is_less);
            sort(v, i + 1, right, is_less);
        }
    }
    fn choose_pivot<T, F>(v: &mut [T], is_less: &mut F) -> (usize, bool)
        where F: FnMut(&T, &T) -> bool {
        // Minimum length to choose the median-of-medians method.
        // Shorter slices use the simple median-of-three method.
        const SHORTEST_MEDIAN_OF_MEDIANS: usize = 50;
        // Maximum number of swaps that can be performed in this function.
        const MAX_SWAPS: usize = 4 * 3;

        let len = v.len();

        // Three indices near which we are going to choose a pivot.
        let mut a = len / 4 * 1;
        let mut b = len / 4 * 2;
        let mut c = len / 4 * 3;

        // Counts the total number of swaps we are about to perform while sorting indices.
        let mut swaps = 0;

        if len >= 8 {
            // Swaps indices so that `v[a] <= v[b]`.
            let mut sort2 = |a: &mut usize, b: &mut usize| unsafe {
                if is_less(v.get_unchecked(*b), v.get_unchecked(*a)) {
                    ptr::swap(a, b);
                    swaps += 1;
                }
            };

            // Swaps indices so that `v[a] <= v[b] <= v[c]`.
            let mut sort3 = |a: &mut usize, b: &mut usize, c: &mut usize| {
                sort2(a, b);
                sort2(b, c);
                sort2(a, b);
            };

            if len >= SHORTEST_MEDIAN_OF_MEDIANS {
                // Finds the median of `v[a - 1], v[a], v[a + 1]` and stores the index into `a`.
                let mut sort_adjacent = |a: &mut usize| {
                    let tmp = *a;
                    sort3(&mut (tmp - 1), a, &mut (tmp + 1));
                };

                // Find medians in the neighborhoods of `a`, `b`, and `c`.
                sort_adjacent(&mut a);
                sort_adjacent(&mut b);
                sort_adjacent(&mut c);
            }

            // Find the median among `a`, `b`, and `c`.
            sort3(&mut a, &mut b, &mut c);
        }

        if swaps < MAX_SWAPS {
            (b, swaps == 0)
        } else {
            // The maximum number of swaps was performed. Chances are the slice is descending or mostly
            // descending, so reversing will probably help sort it faster.
            v.reverse();
            (len - 1 - b, true)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut v = vec![81, 94, 11, 96, 12, 35, 17, 95, 28, 58, 41, 75, 15];
        shell_sort(&mut v, &mut |a, b| a.lt(b));
        assert_eq!(vec![11, 12, 15, 17, 28, 35, 41, 58, 75, 81, 94, 95, 96], v);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v = [81, 94, 11, 96, 12, 35, 17, 95, 28, 58, 41, 75, 15];
        insertion_sort(&mut v, &mut |a, b| a.lt(b));
        assert_eq!([11, 12, 15, 17, 28, 35, 41, 58, 75, 81, 94, 95, 96], v);
    }

    #[test]
    fn test_heap_sort() {
        let mut v = [81, 94, 11, 96, 12, 35, 17, 95, 28, 58, 41, 75, 15];
        heapsort(&mut v, &mut |a, b| a.lt(b));
        assert_eq!([11, 12, 15, 17, 28, 35, 41, 58, 75, 81, 94, 95, 96], v);
    }

    #[test]
    fn test_merge_sort() {
        let mut v = [81, 94, 11, 96, 12, 35, 17, 95, 28, 58, 41, 75, 15];
        merge_sort(&mut v, &mut |a, b| a.lt(b));
        assert_eq!([11, 12, 15, 17, 28, 35, 41, 58, 75, 81, 94, 95, 96], v);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = [81, 94, 11, 96, 12, 35, 17, 95, 28, 58, 41, 75, 15];
        quick_sort(&mut v, &mut |a, b| a.lt(b));
        assert_eq!([11, 12, 15, 17, 28, 35, 41, 58, 75, 81, 94, 95, 96], v);
    }
}