use std::collections::{BinaryHeap, HashMap};
use std::fmt::Binary;

struct Solution;

impl Solution {
    // time complexity : O(n) if max_frequency and len of item_with_frequency are nearly same O(n^2)
    // space complexity : 0(n)
    pub fn top_k_frequent_using_hashmap_only(nums: Vec<i32>, k: i32) -> Vec<i32> {
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
                max_frequency = Some(*value)
            }
        }

        'outer: for i in (1..=max_frequency.unwrap()).rev() {
            for (key, value) in &item_with_frequency {
                if value == &i {
                    result.push(*key);
                    if result.len() == k as usize {
                        break 'outer;
                    }
                }
            }
        }

        result
    }

    // time complexity : O(n log n)
    // space complexity : 0(n)
    pub fn top_k_frequent_using_binary_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // store frequency of items
        let mut item_with_frequency: HashMap<i32, u32> = HashMap::new();
        let mut result: Vec<i32> = vec![];

        for i in nums {
            item_with_frequency
                .entry(i)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        /// store (frequency, item) pair in max_heap
        let mut my_binary_heap: BinaryHeap<(u32, i32)> = BinaryHeap::new();

        for (key, value) in &item_with_frequency {
            my_binary_heap.push((*value, *key))
        }

        while let Some(item) = my_binary_heap.pop(){
            result.push(item.1);
            if result.len() == k as usize {
                break
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works_hash_map() {
        let actual =
            Solution::top_k_frequent_using_hashmap_only(vec![1, 2, 2, 2, 2, 3, 3, 3, 3], 2);
        let expected = [vec![2, 3], vec![3, 2]];
        // asserting with any of pair because hash_map does not maintain order
        assert!(expected.iter().any(|e| *e == actual));

        let actual = Solution::top_k_frequent_using_hashmap_only(vec![1, 2, 2, 2, 2, 3, 3, 3], 1);
        let expected = vec![2];
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_works_binary_heap() {
        let actual = Solution::top_k_frequent_using_binary_heap(vec![1, 2, 2, 2, 2, 3, 3, 3, 3], 2);
        let expected = [vec![2, 3], vec![3, 2]];
        // asserting with any of pair because hash_map does not maintain order
        assert!(expected.iter().any(|e| *e == actual));

        let actual = Solution::top_k_frequent_using_binary_heap(vec![1, 2, 2, 2, 2, 3, 3, 3], 1);
        let expected = vec![2];
        assert_eq!(expected, actual);
    }
}
