use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        // Self::solution_v1(target, position, speed)
        Self::solution_v2(target, position, speed)
    }

    // runtime: 43ms, memory: 5.4 MB
    fn solution_v1(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut speed_map = HashMap::<i32, i32>::new();
        for i in 0..position.len() {
            speed_map.insert(position[i], speed[i]);
        }

        let mut sorted_p = position.clone();
        sorted_p.sort();

        let mut count = 0;
        let mut max_step = 0f32;

        while let Some(p) = sorted_p.pop() {
            let cur_step = (target - p) as f32 / *speed_map.get(&p).unwrap() as f32;
            if max_step < cur_step {
                count += 1;
                max_step = cur_step;
            }
        }

        count
    }

    // runtime: 30 ms, memory: 3.68 MB
    fn solution_v2(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<_> = position.into_iter().zip(speed.into_iter()).collect();
        cars.sort_by(|a, b| b.cmp(a));

        let mut count = 0;
        let mut max_step = 0f32;

        for (p, s) in cars {
            let cur_step = (target - p) as f32 / s as f32;
            if max_step < cur_step {
                max_step = cur_step;
                count += 1;
            }
        }

        count
    }
}
