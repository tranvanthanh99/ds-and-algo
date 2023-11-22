pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut h = std::collections::HashMap::<char, i32>::new();

        for l in s.chars() {
            h.entry(l).and_modify(|x| *x += 1).or_insert(1);
        }

        for l in t.chars() {
            h.entry(l).and_modify(|x| *x -= 1).or_insert(-1);
        }

        h.iter().filter(|x| *x.1 != 0).count() == 0
    }
}
