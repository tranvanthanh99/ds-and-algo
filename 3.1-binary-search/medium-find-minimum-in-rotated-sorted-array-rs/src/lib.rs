pub struct Solution {}

impl Solution {
    // runtime: 1ms, memory: 2.1 MB
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.first().unwrap() <= nums.last().unwrap() {
            return *nums.first().unwrap();
        }
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;

            if nums[m] < nums[0] && nums[m] < *nums.last().unwrap() {
                r = m;
            } else if nums[m] >= nums[0] && nums[m] > *nums.last().unwrap() {
                l = m + 1;
            }
        }

        nums[r]
    }
}