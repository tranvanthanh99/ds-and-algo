pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        Self::solution_v1(heights)
    }

    // brute force - runtime: 1708 ms, memory: 3.1 MB
    fn solution_v1(heights: Vec<i32>) -> i32 {
        let mut max_area = heights[0];

        for (i, h) in heights.iter().enumerate() {
            let mut r = i + 1;
            let mut min_h = *h;

            loop {
                if ((heights.len() - i) as i32 * min_h < max_area && i > 0) || r >= heights.len() {
                    break;
                }

                if min_h > heights[r] {
                    min_h = heights[r];
                }

                if max_area < heights[r] {
                    max_area = heights[r];
                }

                if max_area < min_h * (r - i + 1) as i32 {
                    max_area = min_h * (r - i + 1) as i32;
                }

                r += 1;
            }
        }

        max_area
    }

    fn solution_v2(heights: Vec<i32>) -> i32 {
        
    }
}
