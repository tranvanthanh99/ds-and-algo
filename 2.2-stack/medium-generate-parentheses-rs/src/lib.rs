pub struct Solution {}

impl Solution {
    // runtime: 1 ms, memory: 2.12 MB
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::<String>::new();
        let mut stack = Vec::<(String, i32, i32)>::new();
        stack.push(("(".to_string(), 1, 0));

        while let Some((s, l, r)) = stack.pop() {
            if l == n && r == n {
                res.push(s);
                continue;
            }

            if r < l && r + 1 <= n {
                stack.push((format!("{s})"), l, r + 1));
            }
            if l < n {
                stack.push((format!("{s}("), l + 1, r));
            }
        }

        res
    }
}
