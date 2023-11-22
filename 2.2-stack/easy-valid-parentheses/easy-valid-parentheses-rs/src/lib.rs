pub struct Solution {}

impl Solution {
    // runtime: 0 ms, memory: 2.1 MB
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();

        for v in s.chars().into_iter() {
            match v {
                '(' => stack.push(')'),
                ')' => {
                    if stack.pop() != Some(')') {
                        return false;
                    }
                }
                '[' => stack.push(']'),
                ']' => {
                    if stack.pop() != Some(']') {
                        return false;
                    }
                }
                '{' => stack.push('}'),
                '}' => {
                    if stack.pop() != Some('}') {
                        return false;
                    }
                }
                _ => (),
            }
        }

        if !stack.is_empty() {
            return false;
        }

        true
    }
}
