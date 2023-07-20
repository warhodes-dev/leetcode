#![allow(dead_code)]
mod _2_add_two_numbers;
mod _3_longest_substring_without_repeating_characters;
mod _23_merge_k_sorted_lists;
mod _24_swap_nodes_in_pairs;
mod _37_sudoku_solver;
mod _6_zigzag_conversion;
mod fizzbuzz;

mod list {
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            ListNode { 
                val,
                next: None 
            }
        }
    }

    pub struct List;
    impl List {
        pub fn from_str(num: &str) -> Option<Box<ListNode>> {
            let mut prev_node: Option<Box<ListNode>> = None;

            for char in num.chars().rev() {
                let node = ListNode {
                    val: char.to_digit(10).unwrap() as i32,
                    next: prev_node,
                };
                prev_node = Some(Box::new(node));
            }

            prev_node
        }

        pub fn equals(
            mut l1: Option<Box<ListNode>>,
            mut l2: Option<Box<ListNode>>,
        ) -> bool {
            loop {
                match (l1, l2) {
                    (Some(n1), Some(n2)) => {
                        if n1.val != n2.val { 
                            return false 
                        } else {
                            l1 = n1.next;
                            l2 = n2.next;
                        }
                    },
                    (None, None) => {
                        return true;
                    },
                    _ => { return false; },
                }
            }
        }
    }
}