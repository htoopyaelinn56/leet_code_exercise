use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut s_frequency: HashMap<char, u32> = HashMap::new();
        let mut t_frequency : HashMap<char, u32> = HashMap::new();

        for s in s.chars() {
            s_frequency.entry(s).and_modify(|e| *e += 1).or_insert(1);
        }

        for t in t.chars() {
            t_frequency.entry(t).and_modify(|e| *e += 1).or_insert(1);
        }

        for (k, v) in &s_frequency {
            if t_frequency.get(&k) != Some(&v) {
                return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::valid_anagram::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()), true);
        assert_eq!(Solution::is_anagram("rat".to_string(), "car".to_string()), false);
        assert_eq!(Solution::is_anagram("ab".to_string(), "a".to_string()), false);
    }
}