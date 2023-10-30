use std::cmp::min;

pub struct Solution {}

impl Solution {
    // runtime: 6 ms, memory: 2.9 MB
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_area = 0;

        while l < r {
            let cur_area = min(height[l], height[r]) * (r - l) as i32;
            if cur_area > max_area {
                max_area = cur_area;
            }

            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        max_area
    }
}
