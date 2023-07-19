struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {

    }

    fn sudoku_recursive_solve() {

    }
}

#[cfg(test)]
mod _37 {
    use crate::_37_sudoku_solver::Solution;
    #[test]
    fn case1() {
        let mut board = vec![vec!['5','3','.','.','7','.','.','.','.'],
                             vec!['6','.','.','1','9','5','.','.','.'],
                             vec!['.','9','8','.','.','.','.','6','.'],
                             vec!['8','.','.','.','6','.','.','.','3'],
                             vec!['4','.','.','8','.','3','.','.','1'],
                             vec!['7','.','.','.','2','.','.','.','6'],
                             vec!['.','6','.','.','.','.','2','8','.'],
                             vec!['.','.','.','4','1','9','.','.','5'],
                             vec!['.','.','.','.','8','.','.','7','9']];
        Solution::solve_sudoku(&mut board);
        let correct = vec![vec!['5','3','4','6','7','8','9','1','2'],
                           vec!['6','7','2','1','9','5','3','4','8'],
                           vec!['1','9','8','3','4','2','5','6','7'],
                           vec!['8','5','9','7','6','1','4','2','3'],
                           vec!['4','2','6','8','5','3','7','9','1'],
                           vec!['7','1','3','9','2','4','8','5','6'],
                           vec!['9','6','1','5','3','7','2','8','4'],
                           vec!['2','8','7','4','1','9','6','3','5'],
                           vec!['3','4','5','2','8','6','1','7','9']];
        assert_eq!(board, correct);
    }
}