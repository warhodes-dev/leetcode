use crate::list::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // This is the current layout of our first two items:
        // +--------+      +------------+
        // | val: 1 |      | val: 2     |
        // | next:-------->| next:      |
        // |        |      | Option(..) |
        // +--------+      +------------+

        // We take ownership of right *away from* left. Doing so
        // replaces left.next with None. We now have:
        // 
        //    left            right
        //      |               |
        //      V               V
        // +--------+     +------------+
        // | val: 1 |     | val: 2     |
        // | next:  |     | next:      |
        // | None   |     | Option(..) |
        // +--------+     +------------+
        if let Some(left) = &mut head {
            if let Some(mut right) = left.next.take() {

                // Both nodes are SWAPPED via memswap. We now have:
                // 
                //    left             right
                //      |                |
                //      V                V
                // +------------+    +--------+
                // | val: 2     |    | val: 1 |
                // | next:      |    | next:  |
                // | Option(..) |    | None   |
                // +------------+    +--------+
                std::mem::swap(left, &mut right);

                // left.next represents the entire remaining list, which 
                // we don't want to associate with left. We should take() 
                // ownership of it for later 
                // 
                //    left         right      
                //      |            |        remaining_list: Option(..)
                //      V            V
                // +--------+    +--------+
                // | val: 2 |    | val: 1 |
                // | next:  |    | next:  |
                // | None   |    | None   |
                // +--------+    +--------+
                let remaining_list = left.next.take();

                // Finally, we must recursively swap all subsequent pairs.
                // Items 0 and 1 have already been swapped, so next we should
                // swap items 2 and 3 (right.next & right.next.next). Simply set
                // right.next to be the pair_swapped list. The following recursive 
                // definition of our pairs is the solution:
                //
                //    left         right
                //      |            |
                //      V            V
                // +--------+    +--------+
                // | val: 2 |    | val: 1 |
                // | next:------>| next:  |
                // |        |    | swap_pairs(remaining_list) 
                // +--------+    +--------+
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