pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut e = std::collections::HashSet::<i32>::new();

        for n in nums.into_iter() {
            if !e.insert(n) {
                return true;
            }
        }

        false
    }
}
