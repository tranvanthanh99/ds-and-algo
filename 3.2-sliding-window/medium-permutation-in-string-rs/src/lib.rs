use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    // sliding window - runtime: 4 ms, memory: 2.16 MB
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = HashMap::<u8, i16>::new();
        let mut count = s1.len();
        let mut start = 0;
        
        // o(s1.len())
        for &c in s1.as_bytes().iter() {
            *map.entry(c).or_default() += 1;
        }

        // o(s2.len())
        for (end, &c) in s2.as_bytes().iter().enumerate() {
            match map.get(&c) {
                Some(&v) => {
                    map.insert(c, v - 1);

                    if v > 0 {
                        count -= 1;
                    }
                }
                _ => (),
            }

            if end + 1 - start == s1.len() {
                if count == 0 {
                    return true;
                } else {
                    match map.get(&s2.as_bytes()[start]) {
                        Some(&v) => {
                            map.insert(s2.as_bytes()[start], v + 1);
                            if v >= 0 {
                                count += 1;
                            }
                        }
                        _ => (),
                    }
                    start += 1;
                }
            }
        }

        false
    }
}
