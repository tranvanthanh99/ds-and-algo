use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // Self::solution_v1(numbers, target)
        Self::solution_v2(numbers, target)
    }

    // hash map runtime: 3ms, memory: 2.4MB
    fn solution_v1(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, usize>::new();

        for i in 1..numbers.len() {
            map.insert(numbers[i - 1], i - 1);

            if map.contains_key(&(target - numbers[i])) {
                return vec![
                    *map.get(&(target - numbers[i])).unwrap() as i32 + 1,
                    i as i32 + 1,
                ];
            }
        }

        vec![]
    }

    // two pointer runtime: 1ms, memory: 2.2MB
    fn solution_v2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);

        while l < r {
            if numbers[l] + numbers[r] < target {
                l += 1;
            } else if numbers[l] + numbers[r] > target {
                r -= 1;
            } else {
                return vec![l as i32 + 1, r as i32 + 1];
            }
        }

        unreachable!("invalid test case")
    }
}
