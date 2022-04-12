use std::collections::HashSet;

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut row_sudoku: HashSet<char> = HashSet::new();
    let mut col_sudoku: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut sub_sudoku: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    for r in 0..9 {
        row_sudoku.clear();
        let order_base = 3 * (r / 3);
        for c in 0..9 {
            let current = &board[r][c];
            let order_sub_sudoku = c / 3 + order_base;
            if row_sudoku.contains(current)
                || col_sudoku[c].contains(current)
                || sub_sudoku[order_sub_sudoku].contains(current)
            {
                return false;
            }
            if *current != '.' {
                row_sudoku.insert(*current);
                col_sudoku[c].insert(*current);
                sub_sudoku[order_sub_sudoku].insert(*current);
            }
        }
    }
    true
}
