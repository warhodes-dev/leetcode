use crate::list::ListNode;

struct Solution;

// Solution:
impl Solution {
    pub fn add_two_numbers(
        list_1: Option<Box<ListNode>>,
        list_2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut node_ptr_1 = list_1;
        let mut node_ptr_2 = list_2;

        let mut carry = 0;

        let mut sentinel = Box::new(ListNode::new(-1));
        let mut tail = &mut sentinel;

        while node_ptr_1.is_some() || node_ptr_2.is_some() || carry != 0 {
            let val_1 = node_ptr_1          // node_ptr_1 is an Option<Box<ListNode>>
                .take()                     // Take the Option out of the owning variable (node_ptr_1) *(why? see note)
                .map_or(0, |node| {         // If the Option was None, return 0. 
                    node_ptr_1 = node.next; // Otherwise, mutate node_ptr_1 to be node.next ...
                    node.val                //     and return node.val
                });                         // val_1 will become the value from node.val, and node_ptr_1 will either be None or node.next
            let val_2 = node_ptr_2.take().map_or(0, |node| { node_ptr_2 = node.next; node.val });
            let sum = val_1 + val_2 + carry;

            let val = sum % 10;
            carry = sum / 10;

            let new_node = Some(Box::new(ListNode::new(val)));
            tail.next = new_node;
            tail = tail.next.as_mut().unwrap();
        }

        sentinel.next
    }
    // * as_ref() or as_mut() will fail, because you can't borrow node_ptr_1 and then immediately move it with the closure. You 
    //   must move the option out of node_ptr_1 without destroying the variable, because that variable will be used in the next
    //   iteration of the loop for it's conditional check (node_ptr_1.is_some()). 'take()' performs an 'Indiana Jones switcheroo'
    //   in which the data is moved but replaced with None to maintain type and borrow integrity. We can then set node_ptr_1
    //   inside the closure safely. This is really just a common borrowck workaround.
}

// Test cases:
#[cfg(test)]
mod _2 {
    use crate::_2_add_two_numbers::Solution;
    use crate::list::List;
    #[test]
    fn case1() {
        let l1 = List::from_str("243");
        let l2 = List::from_str("564");
        let sol = List::from_str("708");
        let ans = Solution::add_two_numbers(l1, l2);
        assert!(List::equals(ans, sol))
    }

    #[test]
    fn case2() {
        let l1 = List::from_str("0");
        let l2 = List::from_str("0");
        let sol = List::from_str("0");
        let ans = Solution::add_two_numbers(l1, l2);
        assert!(List::equals(ans, sol))
    }
    
    #[test]
    fn case3() {
        let l1 = List::from_str("9999999");
        let l2 = List::from_str("9999");
        let sol = List::from_str("89990001");
        let ans = Solution::add_two_numbers(l1, l2);
        assert!(List::equals(ans, sol))
    }
}