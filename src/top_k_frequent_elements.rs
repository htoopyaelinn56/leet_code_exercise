use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent_using_hashmap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // store frequency of items
        let mut item_with_frequency: HashMap<i32, u32> = HashMap::new();
        let mut result: Vec<i32> = vec![];

        for i in nums {
            item_with_frequency
                .entry(i)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        // optional tuple
        let mut max_frequency: Option<u32> = None;

        for (key, value) in &item_with_frequency {
            if max_frequency == None || value > &max_frequency.unwrap() {
                max_frequency = Some(value.clone())
            }
        }

        'outer: for i in (1..=max_frequency.unwrap()).rev() {
            for (key, value) in &item_with_frequency {
                if value == &i {
                    result.push(key.clone());
                    if result.len() == k as usize {
                        break 'outer;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let actual = Solution::top_k_frequent_using_hashmap(vec![1, 2, 2, 2, 2, 3, 3, 3, 3], 2);
        let expected = [vec![2, 3], vec![3, 2]];
        // asserting with any of pair because hash_map does not maintain order
        assert!(expected.iter().any(|e| *e == actual));

        let actual = Solution::top_k_frequent_using_hashmap(vec![1, 2, 2, 2, 2, 3, 3, 3], 1);
        let expected = vec![2];
        assert_eq!(expected, actual);
    }
}
