pub struct Solution {}

impl Solution {
    // runtime: 0 ms, memory: 2.2 MB
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut count = 0;
        let mut max = 0;

        for i in 1..height.len() {
            if height[i] > height[max] {
                max = i;
            }
        }
        let mut p = 0;

        for r in 1..max {
            if height[r] < height[p] {
                count += height[p] - height[r];
            } else {
                p = r;
            }
        }

        p = height.len() - 1;

        for l in (max..height.len() - 1).rev() {
            if height[l] < height[p] {
                count += height[p] - height[l];
            } else {
                p = l;
            }
        }

        count
    }
}
