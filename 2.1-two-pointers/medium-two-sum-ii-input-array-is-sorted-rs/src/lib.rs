use std::collections::{HashMap, HashSet};

pub struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        Self::solution_v1(numbers, target)
    }

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
}
