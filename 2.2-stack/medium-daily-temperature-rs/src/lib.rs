pub struct Solution {}

impl Solution {
    // runtime: 47 ms, memory: 4.34 MB
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stacks = Vec::<usize>::new();

        for i in 0..temperatures.len() {
            while let Some(last) = stacks.last() {
                if temperatures[i] > temperatures[*last] {
                    res[*last] = (i - last) as i32;
                    stacks.pop();
                } else {
                    break;
                }
            }
            stacks.push(i);
        }

        res
    }
}