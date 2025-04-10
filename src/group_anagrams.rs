use std::collections::HashMap;

struct Solution;

impl Solution {
    const ASCII_SMALL_A: u32 = 97;
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // key is array with len of 26 to store frequency of characters,
        // value will be the array of those anagrams
        let mut my_hash_map: HashMap<[u32; 26], Vec<String>> = HashMap::new();

        for i in strs {
            let mut frequencies: [u32; 26] = [0; 26];
            for j in i.chars().into_iter() {
                let found_char_index = (j as u32 - Self::ASCII_SMALL_A) as usize;
                frequencies[found_char_index] = frequencies[found_char_index] + 1;
            }
            my_hash_map
                .entry(frequencies)
                .and_modify(|e| e.push(i.clone()))
                .or_insert(vec![i]);
        }

        my_hash_map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_group_anagrams() {
        let mut expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
        ];
        for inner_vec in &mut expected {
            inner_vec.sort();
        }
        expected.sort(); // Sort the outer vector as well for consistent comparison

        let mut actual = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        for inner_vec in &mut actual {
            inner_vec.sort();
        }
        actual.sort(); // Sort the outer vector as well

        assert_eq!(actual, expected);
    }
}
