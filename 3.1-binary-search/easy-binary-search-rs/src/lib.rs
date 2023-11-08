pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if nums[m] > target {
                if m == 0 {
                    break;
                }
                r = m - 1;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                return m as i32;
            }
        }
        -1i32
    }
}
