// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
// such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

struct Solution {}
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let n = nums.len();

        // 1. Sort the array
        let mut nums = nums.clone();
        nums.sort();

        // 2. Iterate through the array with pointer 'i'
        for i in 0..n {
            // Skip duplicates for the first element (nums[i])
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            // Use two pointers for the remaining elements
            let mut left = i + 1;
            let mut right = n - 1;

            // 3. Two-pointer scan
            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];

                if current_sum == 0 {
                    // Found a triplet that sums to zero
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    // Skip duplicates for the second (nums[left]) and third (nums[right]) elements
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    // Move pointers inward
                    left += 1;
                    right -= 1;
                } else if current_sum < 0 {
                    // Sum is too small, move left pointer to increase sum
                    left += 1;
                } else {
                    // Sum is too large, move right pointer to decrease sum
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::three_sum::Solution;

    #[test]
    fn it_works() {
        println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])); // [[-1, -1, 2], [-1, 0, 1]]
        println!("{:?}", Solution::three_sum(vec![0, 1, 1])); // []
        println!("{:?}", Solution::three_sum(vec![0, 0, 0])); // [[0, 0, 0]]
    }
}
