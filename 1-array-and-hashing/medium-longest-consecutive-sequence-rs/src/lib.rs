pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // Self::solution_v1(nums)
        Self::solution_v2(nums)
    }

    // runtime 37ms, memory 3.9MB
    fn solution_v2(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut set = HashSet::<i32>::new();

        for i in nums.iter() {
            set.insert(*i);
        }

        let mut res = 0;

        for n in nums.iter() {
            if !set.contains(&n) {
                continue;
            }

            let mut l = *n;
            let mut r = *n;

            while set.contains(&(l - 1)) {
                set.remove(&l);
                l -= 1;
            }

            while set.contains(&(r + 1)) {
                set.remove(&r);
                r += 1;
            }

            if r - l + 1 > res {
                res = r - l + 1;
            }
        }

        res
    }

    fn solution_v1(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let (mut min, mut max) = (i32::MAX, i32::MIN);
        for n in nums.iter() {
            if max < *n {
                max = *n;
            }
            if min > *n {
                min = *n;
            }
        }

        let mut count = 0;
        let mut max_count = 0;
        if min > 0 {
            let mut e_nums = vec![0u8; (max + 1) as usize];
            for n in nums.iter() {
                e_nums[*n as usize] = 1;
            }

            print!("{:?}", e_nums);

            for i in e_nums.iter() {
                if *i == 1u8 {
                    count += 1;
                } else {
                    count = 0;
                }
                if count > max_count {
                    max_count = count;
                }
            }
        } else {
            let mut p_nums = vec![0u8; (max + 1) as usize];
            let mut n_nums = vec![0u8; (min * -1) as usize];

            for n in nums.iter() {
                if *n >= 0 {
                    p_nums[*n as usize] = 1;
                } else {
                    n_nums[(-1 * (*n + 1)) as usize] = 1;
                }
            }

            for i in n_nums.iter().rev() {
                if *i == 1u8 {
                    count += 1;
                } else {
                    count = 0;
                }
                if count > max_count {
                    max_count = count;
                }
            }

            for i in p_nums.iter() {
                if *i == 1u8 {
                    count += 1;
                } else {
                    count = 0;
                }
                if count > max_count {
                    max_count = count;
                }
            }
        }

        max_count
    }
}
