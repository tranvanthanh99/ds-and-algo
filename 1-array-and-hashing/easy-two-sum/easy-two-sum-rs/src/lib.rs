pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h = std::collections::HashMap::<i32, usize>::new();
        
        for i in 1..nums.len() {
            h.insert(nums[i - 1], i - 1);

            match h.get(&(target - nums[i])) {
                Some(n) => return vec![*n as i32, i as i32],
                None => continue,
            }
        }

        vec![]
    }
}
