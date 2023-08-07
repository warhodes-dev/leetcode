use crate::list::ListNode;

struct Solution;

type OptionNode = Option<Box<ListNode>>;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;

        // Strategy: Increment fast until end of list. Increment slow every *other*
        // time fast is incremented.
        let mut alt = [false, true].iter().cycle();
        while let Some(node) = fast.as_deref() {
            fast = &node.next;
            if *alt.next().unwrap() {
                slow = &slow.as_ref().unwrap().next;
            }
        }

        // Awkwardly the solution expects an owned copy of the target node
        slow.clone()
    }
}

#[cfg(test)]
mod _876 {
    use crate::_876_middle_of_the_linked_list::Solution;
    use crate::list::List;

    #[test]
    fn case1() {
        let input = List::from_str("12345");
        let ans = Solution::middle_node(input);
        let correct = List::from_str("345");
        assert!(List::equals(ans, correct));
    }

    #[test]
    fn case2() {
        let input = List::from_str("123456");
        let ans = Solution::middle_node(input);
        let correct = List::from_str("456");
        assert!(List::equals(ans, correct));
    }
}