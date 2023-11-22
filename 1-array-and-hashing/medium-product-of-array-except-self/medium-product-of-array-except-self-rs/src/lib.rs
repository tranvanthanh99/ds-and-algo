pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        Self::solution(nums)
    }

    fn solution(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        if length == 0 {
            return vec![];
        }
        
        let mut out = vec![1i32; nums.len()];

        for i in 0..length - 1 {
            out[i + 1] = nums[i] * out[i];
        }

        let mut postfix = 1;
        for i in (0..length - 1).rev() {
            postfix *= nums[i + 1];
            out[i] *= postfix;
        }

        out
    }
}
