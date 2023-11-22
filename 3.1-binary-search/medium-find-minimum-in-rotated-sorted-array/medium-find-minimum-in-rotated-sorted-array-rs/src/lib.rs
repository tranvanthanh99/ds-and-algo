pub struct Solution {}

impl Solution {
    // runtime: 1ms, memory: 2.1 MB
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.first().unwrap() <= nums.last().unwrap() {
            return *nums.first().unwrap();
        }
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let m = (l + r) / 2;

            if nums[m] < nums[0] && nums[m] < *nums.last().unwrap() {
                r = m;
            } else if nums[m] >= nums[0] && nums[m] > *nums.last().unwrap() {
                l = m + 1;
            }
        }

        nums[r]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        assert_eq!(super::Solution::find_min(vec![3, 4, 5, 1, 2]), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(super::Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0)
    }
    #[test]
    fn test_3() {
        assert_eq!(super::Solution::find_min(vec![11, 13, 15, 17]), 11)
    }
    #[test]
    fn test_4() {
        assert_eq!(super::Solution::find_min(vec![1]), 1)
    }
    #[test]
    fn test_5() {
        assert_eq!(super::Solution::find_min(vec![2, 1]), 1)
    }
}
