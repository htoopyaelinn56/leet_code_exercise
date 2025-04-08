use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_hash_map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let result = target - nums[i];
            if my_hash_map.contains_key(&result) {
                return vec![i as i32, *my_hash_map.get(&result).unwrap()];
            }
            my_hash_map.insert(nums[i], i as i32);
        }
        vec![]
    }
}
