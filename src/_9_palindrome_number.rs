struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_string = x.to_string();
        let iter = x_string.chars();
        let iter_reverse = x_string.chars().rev();
        iter.eq(iter_reverse)
    }
}

#[cfg(test)]
mod _9 {
    use crate::_9_palindrome_number::Solution;
    #[test]
    fn case1() {
        assert!(Solution::is_palindrome(121));
    }
    #[test]
    fn case2() {
        assert!(!Solution::is_palindrome(-121));
    }
    #[test]
    fn case3() {
        assert!(!Solution::is_palindrome(10));
    }
}