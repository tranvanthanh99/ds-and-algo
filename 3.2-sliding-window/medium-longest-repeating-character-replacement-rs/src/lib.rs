use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // Self::solution_v1(s, k)
        Self::solution_v2(s, k)
    }

    // sliding window - runtime: 6ms, memory: 2.1 MB
    fn solution_v1(s: String, k: i32) -> i32 {
        let mut map = HashMap::<u8, i32>::new();
        let mut count = 0;
        let mut start = 0;
        let mut ans = 0;

        for (i, &c) in s.as_bytes().iter().enumerate() { // o(n)
            match map.get(&c) {
                Some(&v) => {
                    map.insert(c, v + 1);
                    count = count.max(v + 1);
                }
                None => {
                    map.insert(c, 1);
                    count = count.max(1);
                }
            }

            while k < (i + 1 - start) as i32 - count {
                map.entry(s.as_bytes()[start]).and_modify(|v| {
                    count = count.max(*v);
                    if *v == count {
                        count -= 1;
                    }
                    *v -= 1;
                });
                start += 1;

                count = count.max(*map.get(&s.as_bytes()[i - 1]).unwrap_or(&0));
                count = count.max(*map.get(&s.as_bytes()[start]).unwrap_or(&0));
            }

            ans = ans.max(i - start + 1);
        }

        ans as i32
    }

    // sliding window (optimized) - runtime: 4ms, memory: 2 MB
    fn solution_v2(s: String, k: i32) -> i32 {
        let mut map = HashMap::<u8, i32>::new();
        let mut start = 0;
        let mut count_max = 0;
        let mut ans = 0;

        for (end, &c) in s.as_bytes().iter().enumerate() { // o(n)
            *map.entry(c).or_default() += 1;

            count_max = count_max.max(*map.get(&c).unwrap());

            if (end - start + 1) as i32 - count_max > k {
                *map.entry(s.as_bytes()[start]).or_default() -= 1;

                start += 1;
            }

            ans = ans.max(end - start + 1);
        }

        ans as i32
    }
}
