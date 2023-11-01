pub struct Solution {}

impl Solution {
    // runtime: 2ms, memory: 2.58 MB
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::<i32>::new();

        for t in tokens.iter() {
            if *t == "+" {
                let mut res = stack.pop().unwrap();
                res = stack.pop().unwrap() + res;
                stack.push(res);
            } else if *t == "-" {
                let mut res = stack.pop().unwrap();
                res = stack.pop().unwrap() - res;
                stack.push(res);
            } else if *t == "*" {
                let mut res = stack.pop().unwrap();
                res = stack.pop().unwrap() * res;
                stack.push(res);
            } else if *t == "/" {
                let mut res = stack.pop().unwrap();
                res = stack.pop().unwrap() / res;
                stack.push(res);
            } else {
                stack.push(t.parse::<i32>().unwrap());
            }
        }

        *stack.first().unwrap()
    }
}
