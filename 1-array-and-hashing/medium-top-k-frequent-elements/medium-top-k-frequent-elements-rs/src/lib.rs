pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut h = HashMap::<i32, u32>::new();
        
        for n in nums {
            h.entry(n).and_modify(|v| *v += 1).or_insert(1u32);
        }
        
        let mut sv = h.values().collect::<Vec<&u32>>();
        sv.sort();
        
        let mut o = Vec::<i32>::new();
        for i in h.keys() {
            if h.get(i).unwrap() >= sv[sv.len() - k as usize] {
                o.push(*i);
            }
        }

        o
    }
}
