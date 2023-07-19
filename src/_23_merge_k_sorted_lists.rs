use crate::list::ListNode;

struct Solution;

impl Solution {
    pub fn merge_k_lists(
        lists: Vec<Option<Box<ListNode>>>
    ) -> Option<Box<ListNode>> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut queue = lists.into_iter()
            .filter(Option::is_some)
            .map(|node| Reverse(node))
            .collect::<BinaryHeap<_>>();

        let mut head = None;
        let mut tail = &mut head;
        
        while let Some(Reverse(mut node)) = queue.pop() {

            let next_node = node.as_mut().and_then(|node| node.next.take());
            if next_node.is_some() {
                queue.push(Reverse(next_node));
            }
            
            *tail = node;
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
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
    #[test]
    fn case4() {
        let lists = vec![
            List::from_str("145889"),
            List::from_str("1344445699"),
            List::from_str("2344489"),
        ];
        let correct = List::from_str("11233444444445568889999");
        let ans = Solution::merge_k_lists(lists);
        println!("Correct: {:?}", correct);
        println!("Answer: {:?}", ans);
        assert!(List::equals(ans, correct))
    }
}