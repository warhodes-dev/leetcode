struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_median_sorted_arrays(array_1: Vec<i32>, array_2: Vec<i32>) -> f64 {

        let total_len = array_1.len() + array_2.len();

        todo!();
    }
}

pub struct MinHeap {
    arr: Vec<i32>,
    size: usize,
    capacity: usize,
}

impl MinHeap {
    pub fn parent(i: usize) {
    }
}

#[cfg(test)]
mod _4 {
    use crate::_4_median_of_two_sorted_arrays::Solution;
    #[test]
    fn case1() {
        let ans = Solution::find_median_sorted_arrays(vec![1,3], vec![2]);
        assert_eq!(ans, 2.0);
    }
    #[test]
    fn case2() {
        let ans = Solution::find_median_sorted_arrays(vec![1,2], vec![3,4]);
        assert_eq!(ans, 2.5);
    }
    #[test]
    fn case3() {
        let ans = Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9, 11, 13, 15], vec![2, 4, 6, 8, 10, 12, 14]);
        assert_eq!(ans, 8.0);
    }
}