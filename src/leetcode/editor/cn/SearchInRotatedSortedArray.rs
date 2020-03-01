//! [33]搜索旋转排序数组
//假设按照升序排序的数组在预先未知的某个点上进行了旋转。
//
// ( 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2] )。
//
// 搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。
//
// 你可以假设数组中不存在重复的元素。
//
// 你的算法时间复杂度必须是 O(log n) 级别。
//
// 示例 1:
//
// 输入: nums = [4,5,6,7,0,1,2], target = 0
//输出: 4
//
//
// 示例 2:
//
// 输入: nums = [4,5,6,7,0,1,2], target = 3
//输出: -1
// Related Topics 数组 二分查找

///
///
pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        fn search_r_point(nums: &[i32], start: usize, end: usize) -> usize {
            if start == end {
                start
            } else {
                let mid = (start + end) / 2;
                if nums[start] < nums[mid] {
                    search_r_point(nums, mid, end)
                } else {
                    search_r_point(nums, start, mid)
                }
            }
        }
        fn search_target(nums: &[i32], start: usize, end: usize, target: i32) -> i32 {
            if start == end {
                if nums[start] == target {
                    start as i32
                } else {
                    -1
                }
            } else {
                let mid = (start + end) / 2;
                if nums[mid] < target {
                    search_target(nums, mid + 1, end, target)
                } else {
                    search_target(nums, start, mid, target)
                }
            }
        }
        if nums.is_empty() {
            return -1;
        } else if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        } else if nums[0] <= nums[nums.len() - 1] {
            return search_target(nums.as_slice(), 0, nums.len() - 1, target);
        }
        let rotate_point = search_r_point(nums.as_slice(), 0, nums.len() - 1);
        if nums[rotate_point] == target {
            rotate_point as i32
        } else if target < nums[0] {
            search_target(nums.as_slice(), rotate_point + 1, nums.len() - 1, target)
        } else {
            search_target(nums.as_slice(), 0, rotate_point + 1, target)
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
