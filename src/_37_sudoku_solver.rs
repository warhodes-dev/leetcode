struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        recursive_solve(board);
    }
}

type Board = Vec<Vec<char>>;

fn recursive_solve(board: &mut Board) -> bool {
    for r in 0..9 {
        for c in 0..9 {

            // Recursive case: If there is an unfilled '.', fill it with any
            // number and recurse on the new board state.
            if board[r][c] == '.' {
                for i in 1..=9 {
                    board[r][c] = char::from_digit(i, 10).unwrap();
                    if check_row(board, r)
                    && check_column(board, c)
                    && check_chunk(board, r, c)
                    && recursive_solve(board) {
                        return true;
                    }
                }

                // Failure case: We tried every number and found no solution
                board[r][c] = '.';
                return false;
            }
        }
    }

    // Base case: There were no empty '.' found, so the puzzle is solved
    return true;
}

fn check_row(board: &Board, row_idx: usize) -> bool {
    let mut set = std::collections::HashSet::new();
    let row = &board[row_idx];
    for &cell in row {
        if cell != '.' && !set.insert(cell) {
            return false;
        }
    }
    return true;
}

fn check_column(board: &Board, col_idx: usize) -> bool {
    let mut set = std::collections::HashSet::new();
    for row in board.iter() {
        let cell = row[col_idx];
        if cell != '.' && !set.insert(cell) {
            return false;
        }
    }
    return true;
}

fn check_chunk(board: &Board, row_idx: usize, col_idx: usize) -> bool {
    let mut set = std::collections::HashSet::new();
    let chunk_x = (row_idx / 3) * 3;
    let chunk_y = (col_idx / 3) * 3;

    for x in chunk_x..(chunk_x + 3) {
        for y in chunk_y..(chunk_y + 3) {
            let cell = board[x][y];
            if cell != '.' && !set.insert(cell) {
                return false;
            }
        }
    }
    return true;
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