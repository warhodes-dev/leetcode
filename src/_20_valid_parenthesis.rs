

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {

        let mut stack = Vec::<char>::new();

        for char in s.chars() {
            match char {
                '(' | '[' | '{' => { stack.push(char) },
                ')' if stack.last().is_some() && *stack.last().unwrap() == '(' => { stack.pop(); },
                ']' if stack.last().is_some() && *stack.last().unwrap() == '[' => { stack.pop(); },
                '}' if stack.last().is_some() && *stack.last().unwrap() == '{' => { stack.pop(); },
                _ => { return false; }
            }
        }

        if stack.is_empty() {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod _20 {
    use crate::_20_valid_parenthesis::Solution;
    #[test]
    fn case1() {
        assert!(Solution::is_valid("()".to_string()))
    }
    #[test]
    fn case2() {
        assert!(Solution::is_valid("()[]{}".to_string()))
    }
    #[test]
    fn case3() {
        assert!(!Solution::is_valid("(]".to_string()))
    }
}