// https://leetcode.com/problems/monotonic-array/description/

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut is_decreasing = true;
        let mut is_increasing = true;

        #[allow(clippy::bool_comparison)]
        for index in 1..nums.len() {
            if is_decreasing == true && nums[index - 1] > nums[index] {
                is_decreasing = false;
            }

            if is_increasing == true && nums[index - 1] < nums[index] {
                is_increasing = false;
            }
        }

        is_decreasing || is_increasing
    }
}
