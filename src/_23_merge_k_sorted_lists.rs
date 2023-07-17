use crate::list::ListNode;

struct Solution;

impl Solution {
    pub fn merge_k_lists(
        lists: Vec<Option<Box<ListNode>>>
    ) -> Option<Box<ListNode>> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut queue = lists.into_iter()
            .flatten()
            .map(|node| Reverse(node))
            .collect::<BinaryHeap<_>>();

        let mut sentinel = Box::new(ListNode::new(-1));
        let mut tail = &mut sentinel;
        
        while let Some(Reverse(node)) = queue.pop() {

            if let Some(next_node) = node.next {
                queue.push(Reverse(next_node));
            }
            
            let new_node = Some(Box::new(ListNode::new(node.val)));
            tail.next = new_node;
            tail = tail.next.as_mut().unwrap();
        }

        sentinel.next
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

#[cfg(test)]
mod _23 {
    use crate::_23_merge_k_sorted_lists::Solution;
    use crate::list::List;
    #[test]
    fn case1() {
        let lists = vec![
            List::from_str("145"),
            List::from_str("134"),
            List::from_str("26"),
        ];
        let correct = List::from_str("11234456");
        let ans = Solution::merge_k_lists(lists);
        println!("Correct: {:?}", correct);
        println!("Answer: {:?}", ans);
        assert!(List::equals(ans, correct))
    }
    #[test]
    fn case2() {
        let lists = vec![
        ];
        let correct = List::from_str("");
        let ans = Solution::merge_k_lists(lists);
        assert!(List::equals(ans, correct))
    }
    #[test]
    fn case3() {
        let lists = vec![
            List::from_str("")
        ];
        let correct = List::from_str("");
        let ans = Solution::merge_k_lists(lists);
        assert!(List::equals(ans, correct))
    }
}