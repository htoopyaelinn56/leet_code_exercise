use std::collections::HashMap;
use std::hash::Hash;

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for i in 0..nums.len() {
            let prefix_items = &nums[0..i];
            let suffix_items = &nums[i + 1..nums.iter().len()];

            let product_of_prefix = prefix_items.iter().fold(1, |product, i| product * i);
            let product_of_suffix = suffix_items.iter().fold(1, |product, i| product * i);
            let product_except_self = product_of_prefix * product_of_suffix;

            result.push(product_except_self)
        }
        result
    }

    pub fn product_except_self_optimized(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n]; // Initialize result vector with 1s

        // Calculate prefix products
        let mut prefix_product = 1;
        for i in 0..n {
            result[i] = prefix_product;
            prefix_product *= nums[i];
        }

        // Calculate suffix products and multiply with prefix products
        let mut suffix_product = 1;
        for i in (0..n).rev() {
            result[i] *= suffix_product;
            suffix_product *= nums[i];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn it_works_with_optimized() {
        assert_eq!(
            Solution::product_except_self_optimized(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self_optimized(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
