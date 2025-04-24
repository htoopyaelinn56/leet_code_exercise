struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            let current_sum = numbers[left] + numbers[right];
            if current_sum == target {
                return vec![(left + 1) as i32, (right+1) as i32]
            } else if current_sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::two_sum_2::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![0, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}