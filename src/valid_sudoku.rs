use std::collections::HashSet;

struct Solution;

impl Solution {
    // time complexity : 0(1) for this problem but 0(N) for general sudoku
    // space complexity : 0(1) for this problem but 0(N) for general sudoku
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for i in 0..81 {
            let row = i / 9;
            let col = i % 9;

            let ch = board[row][col];
            if ch == '.' {
                continue;
            }

            let box_index = (row / 3) * 3 + (col / 3);

            if !rows[row].insert(ch) {
                return false;
            } else if !cols[col].insert(ch) {
                return false;
            } else if !boxes[box_index].insert(ch) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_valid_sudoku_true() {
        let board: Vec<Vec<char>> = vec![
            vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
            vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '.', '3'],
            vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
            vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn test_is_valid_sudoku_false() {
        let board: Vec<Vec<char>> = vec![
            vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
            vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '9', '1', '.', '.', '.', '.', '.', '3'],
            vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
            vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }
}
