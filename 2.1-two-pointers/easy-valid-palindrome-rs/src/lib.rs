pub struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        Self::solution_v1(s)
    }

    fn solution_v1(s: String) -> bool {
        let chars = s
            .chars()
            .map(|c| {
                if c as u8 >= 97 && c as u8 <= 122 {
                    return c as u8 - 32;
                }
                c as u8
            })
            .collect::<Vec<u8>>();
        let (mut l, mut r) = (0, chars.len() - 1);

        while l < r {
            if (chars[l] < 65 || chars[l] > 90) && (chars[l] < 48 || chars[l] > 57) {
                l += 1;
                continue;
            }
            if (chars[r] < 65 || chars[r] > 90) && (chars[r] < 48 || chars[r] > 57) {
                r -= 1;
                continue;
            }

            if chars[l] != chars[r] {
                return false;
            }

            r -= 1;
            l += 1;
        }

        true
    }
}
