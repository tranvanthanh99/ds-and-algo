pub struct Solution {}

impl Solution {
    // runttime: 8 ms, memory: 3.1 MB
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = nums[0];
        let mut fast = nums[0];

        // find catchup point
        // 0(n)
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];

            if slow == fast {
                break;
            }
        }

        fast = nums[0];

        // find duplicate
        // o(n)
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        fast
    }
}
