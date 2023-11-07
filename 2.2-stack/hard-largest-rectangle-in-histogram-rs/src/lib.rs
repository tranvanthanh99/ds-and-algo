use std::cmp;

pub struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // Self::solution_v1(heights)
        // Self::solution_v2(heights)
        Self::solution_v3(heights)
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

    // smaller right and smaller left
    // runtime: 14 ms, memory: 5.1 MB
    fn solution_v2(heights: Vec<i32>) -> i32 {
        let mut stack = Vec::<usize>::new();
        let mut l = vec![0; heights.len()];
        let mut r = vec![0; heights.len()];
        let mut max_area = 0;

        for i in 0..heights.len() {
            while let Some(s) = stack.last() {
                if heights[i] <= heights[*s] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.is_empty() {
                l[i] = 0;
            } else {
                l[i] = *stack.last().unwrap() + 1;
            }
            stack.push(i);
        }

        stack.clear();

        for i in (0..heights.len()).rev() {
            while let Some(s) = stack.last() {
                if heights[i] <= heights[*s] {
                    stack.pop();
                } else {
                    break;
                }
            }
            if stack.is_empty() {
                r[i] = heights.len() - 1;
            } else {
                r[i] = *stack.last().unwrap() - 1;
            }
            stack.push(i);
        }

        for i in 0..heights.len() {
            max_area = cmp::max(max_area, heights[i] * (r[i] - l[i] + 1) as i32);
        }

        max_area
    }

    // one stack - runtime: 13 ms, memory: 3.3 MB
    fn solution_v3(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0);
        let mut stack = Vec::<usize>::new();
        let mut max_area = heights[0];

        for (i, &hi) in heights.iter().enumerate() {
            while let Some(&l) = stack.last() {
                if hi > heights[l] {
                    stack.push(i);
                    break;
                } else {
                    stack.pop();
                    max_area = max_area.max(match stack.last() {
                        None => heights[l] * i as i32,
                        Some(&k) => heights[l] * (i - k - 1) as i32,
                    })
                }
            }
            if stack.is_empty() {
                stack.push(i);
            }
        }

        max_area
    }
}
