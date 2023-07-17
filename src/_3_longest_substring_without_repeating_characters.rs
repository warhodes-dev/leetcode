struct Solution;

// Solution:
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = std::collections::HashMap::<char, i32>::new();

        let mut max = 0;    // length of longest substring
        let mut start = -1; // `start` of the substring span we're evaluating

        for (idx, char) in s.chars().enumerate() {

            // `old_start` of the substring we *were* evaluating when 
            // char and its index were inserted into the map
            if let Some(old_start) = map.insert(char, idx as i32) {

                // `start` should never be reduced to a lower idx.
                //
                // Consider this example: "abba"
                //  By the time the loop reaches the second 'b', `start`
                //  will updated to 1 to capture that the substring "b"
                //  spans 1 -> 2 for a length of 1. When the loop reaches
                //  the second 'a', `old_start` will be -1. If we reset
                //  `start` back to -1, then the span will be -1 -> 3 for 
                //  a length of 4 which is clearly wrong.
                //
                //  tl:dr if `old_start` < `start`, it's because that span's
                //  start value was made obsolete and should be ignored
                start = std::cmp::max(start, old_start);
            }

            // `max` is the largest span we've seen so far.
            max = std::cmp::max(max, idx as i32 - start);
        }

        max
    }
}

// Test cases:
#[cfg(test)]
mod _3 {
    use crate::_3_longest_substring_without_repeating_characters::Solution;
    #[test]
    fn case1() {
        let input = "abcabcbb";
        let correct = 3;
        let ans = Solution::length_of_longest_substring(input.to_string());
        assert_eq!(ans, correct) 
    }

    #[test]
    fn case2() {
        let input = "bbbbb";
        let correct = 1;
        let ans = Solution::length_of_longest_substring(input.to_string());
        assert_eq!(ans, correct) 
    }

    #[test]
    fn case3() {
        let input = "pwwkew";
        let correct = 3;
        let ans = Solution::length_of_longest_substring(input.to_string());
        assert_eq!(ans, correct) 
    }

    #[test]
    fn case4() {
        let input = "aab";
        let correct = 2;
        let ans = Solution::length_of_longest_substring(input.to_string());
        assert_eq!(ans, correct) 
    }

    #[test]
    fn case5() {
        let input = "abba";
        let correct = 2;
        let ans = Solution::length_of_longest_substring(input.to_string());
        assert_eq!(ans, correct) 
    }
}
