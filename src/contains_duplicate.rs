use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut my_set = HashSet::new();

        for i in 0..nums.len() {
            if my_set.contains(&nums[i]) {
                return true;
            }
            my_set.insert(nums[i]);
        }

        false
    }
}