struct Solution;

impl Solution {
    // Time Complexity : O(N)
    // Space Complexity : O(N)
    pub fn is_palindrome(s: String) -> bool {
        let s1: String = s
            .to_lowercase()
            .chars()
            .rev()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let s2: String = s1.chars().rev().collect();
        s1 == s2
    }
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome::Solution;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome("poop".to_string()), true);
        assert_eq!(Solution::is_palindrome("Abcba".to_string()), true);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
        assert_eq!(Solution::is_palindrome("hello".to_string()), false);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
}
