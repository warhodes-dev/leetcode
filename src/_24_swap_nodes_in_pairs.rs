use crate::list::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        if let Some(left) = &mut head {
            if let Some(mut right) = left.next.take() {
                std::mem::swap(left, &mut right);
                let remaining_list = left.next.take();
                right.next = Solution::swap_pairs(remaining_list);
                left.next = Some(right);
            }
        }

        head
    }
}

#[cfg(test)]
mod _24 {
    use crate::_24_swap_nodes_in_pairs::Solution;
    use crate::list::List;
    #[test]
    fn case1() {
        let list = List::from_str("1234");
        let correct = List::from_str("2143");
        let ans = Solution::swap_pairs(list);
        assert_eq!(ans, correct);
    }
    #[test]
    fn case2() {
        let list = List::from_str("");
        let correct = List::from_str("");
        let ans = Solution::swap_pairs(list);
        assert_eq!(ans, correct);
    }
    #[test]
    fn case3() {
        let list = List::from_str("1");
        let correct = List::from_str("1");
        let ans = Solution::swap_pairs(list);
        assert_eq!(ans, correct);
    }
}