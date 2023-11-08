pub struct Solution {}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = *piles.iter().max().unwrap();

        while l < r {
            let m = (l + r) / 2;
            let mut hours = 0;

            for &p in piles.iter() {
                hours += (p as f64 / m as f64).ceil() as i32;
            }

            if h >= hours {
                r = m;
            } else {
                l = m + 1;
            }
        }

        r
    }
}
