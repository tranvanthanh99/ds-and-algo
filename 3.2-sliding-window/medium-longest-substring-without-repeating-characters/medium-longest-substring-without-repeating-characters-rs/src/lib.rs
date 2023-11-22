use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::<char, usize>::new();
        let mut j = 0;
        let mut ans = 0;

        for (i, c) in s.chars().enumerate() { // o(n)
            match map.get(&c) {
                Some(&v) => {
                    if v < j {
                        ans = ans.max(i - j + 1);
                    }
                    j = j.max(v + 1);
                    map.insert(c, i);
                }
                None => {
                    map.insert(c, i);
                    ans = ans.max(i - j + 1);
                }
            }
        }

        ans as i32
    }
}
