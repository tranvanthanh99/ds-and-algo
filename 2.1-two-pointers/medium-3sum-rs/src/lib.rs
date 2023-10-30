pub struct Solution {}

impl Solution {
    // runtime: 40 ms, memory: 4 MB
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        for i in 0..sorted_nums.len() - 2 {
            if i > 0 && sorted_nums[i] == sorted_nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, sorted_nums.len() - 1);

            while l < r {
                if l > i + 1 && sorted_nums[l] == sorted_nums[l - 1] {
                    l += 1;
                    continue;
                }

                match sorted_nums[i] + sorted_nums[l] + sorted_nums[r] {
                    n if n > 0 => r -= 1,
                    n if n < 0 => l += 1,
                    _ => {
                        res.push(vec![sorted_nums[i], sorted_nums[l], sorted_nums[r]]);
                        l += 1;
                        r -= 1;
                    }
                }
            }
        }

        res
    }
}
