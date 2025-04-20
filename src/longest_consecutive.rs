use std::cmp::max;
use std::collections::HashSet;

struct Solution;
impl Solution {
    // time complexity : O(N)
    // space complexity : O(N)
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut longest_consecutive_count = 0;

        // to gain 0(1) lookups
        let my_set: HashSet<i32> = HashSet::from_iter(nums);

        for i in &my_set {
            let mut consecutive_count = 0;
            if !my_set.contains(&(i - 1)) {
                // O(1) here
                while my_set.contains(&(i + consecutive_count)) {
                    consecutive_count += 1;
                }
                longest_consecutive_count = max(longest_consecutive_count, consecutive_count);
            }
        }

        longest_consecutive_count
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::longest_consecutive(vec![2, 1, 3, 4, 10, 11, 12]),
            4
        );

        assert_eq!(
            Solution::longest_consecutive(vec![
                2, 1, 3, 4, 10, 11, 12, 13, 14, 14, 15, 16, 16, 16, 17
            ]),
            8
        );

        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 2, 5, 4, 6, 1, 1]),
            7
        );
    }
}
