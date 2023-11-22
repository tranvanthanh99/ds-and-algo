pub struct Solution {}

impl Solution {
    // runtime: 1 ms, memory: 2.1 MB
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut l = 0;
        let mut r = matrix.len() - 1;

        while l < r {
            let m = (l + r) / 2;

            if target > *matrix[m].last().unwrap() {
                l = m + 1;
            } else if target < *matrix[m].last().unwrap() {
                r = m;
            } else {
                return true;
            }
        }

        if l == r {
            let ri = l;
            l = 0;
            r = matrix[0].len() - 1;

            while l <= r {
                let m = (l + r) / 2;

                if target > matrix[ri][m] {
                    l = m + 1;
                } else if target < matrix[ri][m] {
                    if m == 0 {
                        break;
                    }
                    r = m - 1;
                } else {
                    return true;
                }
            }
        }

        false
    }
}
