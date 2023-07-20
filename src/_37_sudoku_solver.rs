struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut sudoku = Sudoku::new(board);    
        recursive_solve(&mut sudoku);
    }
}

fn recursive_solve(sudoku: &mut Sudoku) -> bool {
    for r in 0..9 {
        for c in 0..9 {

            // Recursive case: If there is an unfilled '.', fill it with any
            // number and recurse on the new board state.
            if sudoku.board[r][c] == '.' {
                for i in 1..=9 {
                    sudoku.board[r][c] = char::from_digit(i, 10).unwrap();
                    if sudoku.check_row(r)
                    && sudoku.check_column(c)
                    && sudoku.check_chunk_xy(r, c)
                    && recursive_solve(sudoku) {
                        return true;
                    }

                }

                // Failure case: We tried every number and found no solution
                sudoku.board[r][c] = '.';
                return false;
            }
        }
    }

    // Base case: There were no empty '.' found, so the puzzle is solved
    return true;
}

struct Sudoku<'a> {
    board: &'a mut Vec<Vec<char>>
}

impl<'a> Sudoku<'a> {
    fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        Sudoku { board }
    }

    fn check_row(&self, row_idx: usize) -> bool {
        let mut set = std::collections::HashSet::new();
        let row = &self.board[row_idx];
        for &cell in row {
            if cell != '.' && !set.insert(cell) {
                return false;
            }
        }
        return true;
    }

    fn check_column(&self, col_idx: usize) -> bool {
        let mut set = std::collections::HashSet::new();
        for row in self.board.iter() {
            let cell = row[col_idx];
            if cell != '.' && !set.insert(cell) {
                return false;
            }
        }
        return true;
    }

    fn check_chunk_xy(&self, row_idx: usize, col_idx: usize) -> bool {
        let mut set = std::collections::HashSet::new();
        let chunk_x = (row_idx / 3) * 3;
        let chunk_y = (col_idx / 3) * 3;

        for x in chunk_x..(chunk_x + 3) {
            for y in chunk_y..(chunk_y + 3) {
                let cell = self.board[x][y];
                if cell != '.' && !set.insert(cell) {
                    return false;
                }
            }
        }
        return true;
    }

    fn check_chunk(&self, chunk_idx: usize) -> bool {
        if !(0..9).contains(&chunk_idx) {
            panic!("Invalid chunk index: {chunk_idx}");
        }
        let (x, y) = (
            (chunk_idx % 3) * 3,
            (chunk_idx / 3) * 3,
        );
        self.check_chunk_xy(x, y)
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