pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut h = HashMap::<[u8; 26], Vec<String>>::new();

        for s in strs {
            let mut k = [0u8; 26];

            for c in s.chars() {
                k[c as usize - 'a' as usize] += 1;
            }

            h.entry(k).or_default().push(s);
        }

        h.into_values().collect()
    }
}
