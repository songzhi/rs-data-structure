//! [215]数组中的第K个最大元素
//在未排序的数组中找到第 k 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。 
//
// 示例 1: 
//
// 输入: [3,2,1,5,6,4] 和 k = 2
//输出: 5
// 
//
// 示例 2: 
//
// 输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
//输出: 4 
//
// 说明: 
//
// 你可以假设 k 总是有效的，且 1 ≤ k ≤ 数组的长度。 
// Related Topics 堆 分治算法




pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Ordering;
        fn partition(nums: &mut [i32], left: usize, right: usize, pivot_index: usize) -> usize {
            let pivot = nums[pivot_index];
            nums.swap(pivot_index, right);
            let mut store_index = left;
            for i in left..right {
                if nums[i] < pivot {
                    nums.swap(i, store_index);
                    store_index += 1;
                }
            }
            nums.swap(right, store_index);
            store_index
        }
        fn select(nums: &mut [i32], left: usize, right: usize, k_smallest: usize) -> i32 {
            if left == right { return nums[left]; }
            let mut pivot_index = (left + right) / 2;
            pivot_index = partition(nums, left, right, pivot_index);
            match k_smallest.cmp(&pivot_index) {
                Ordering::Less => select(nums, left, pivot_index - 1, k_smallest),
                Ordering::Equal => nums[k_smallest],
                Ordering::Greater => select(nums, pivot_index + 1, right, k_smallest),
            }
        }
        let right = nums.len() - 1;
        let k_smallest = nums.len() - k as usize;
        select(&mut nums, 0, right, k_smallest)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
