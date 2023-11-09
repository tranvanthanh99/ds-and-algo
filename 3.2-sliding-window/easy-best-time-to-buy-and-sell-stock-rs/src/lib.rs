pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        Self::solution_v1(prices)
    }

    // two pointers - runtime: 10 ms, memory: 3 MB
    fn solution_v1(prices: Vec<i32>) -> i32 {
        let mut l = prices[0];
        let mut r = prices[0];
        let mut profit = 0;

        for &p in prices.iter().skip(1) {
            if p < l {
                l = p;
                r = p;
            }
            if p > r {
                r = p;
            }
            if profit < r - l {
                profit = r - l;
            }
        }

        profit
    }
}
