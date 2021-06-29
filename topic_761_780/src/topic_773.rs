use std::collections::HashSet;

const MOVE_STEP: [&str; 4] = ["UP", "DOWN", "LEFT", "RIGHT"];


pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
    let mut rst = 0;
    let mut visited_set = HashSet::new();
    let mut start = HashSet::new();
    start.insert(parse_string(&board));
    let mut end = HashSet::new();
    end.insert(String::from("123450"));
    while start.len() > 0 && end.len() > 0 {
        let mut tmp = HashSet::new();
        for s in start.iter() {
            if end.contains(s) {
                return rst;
            } else {
                visited_set.insert(s.clone());
                let mut now_board = parse_vec(s);
                let mut row = 0usize;
                let mut col = 0usize;
                'find_zero: for i in 0..2 {
                    for j in 0..3 {
                        if now_board[i][j] == 0 {
                            row = i;
                            col = j;
                            break 'find_zero;
                        }
                    }
                }
                for i in 0..MOVE_STEP.len() {
                    let new_s = get_str(&mut now_board, row, col, MOVE_STEP[i]);
                    if let Some(v) = new_s {
                        if !visited_set.contains(&v) {
                            tmp.insert(v);
                        }
                    }
                }
            }
        }
        rst += 1;
        start = end;
        end = tmp;
    }
    -1
}


/// 搬动 0 的位置，获取新的字符串，这里要注意，之所以要进行交换两次，因为我们要保证数组的不变，以便给下一种搬动使用
pub fn get_str(board: &mut Vec<Vec<i32>>, row: usize, col: usize, step: &str) -> Option<String> {
    match step {
        "UP" => {
            if row == 1 {
                swap(board, row, col, 0, col);
                let rst = parse_string(board);
                swap(board, row, col, 0, col);
                Some(rst)
            } else {
                None
            }
        }
        "DOWN" => {
            if row == 0 {
                swap(board, row, col, 1, col);
                let rst = parse_string(board);
                swap(board, row, col, 1, col);
                Some(rst)
            } else {
                None
            }
        }
        "LEFT" => {
            if col > 0 {
                swap(board, row, col, row, col - 1);
                let rst = parse_string(board);
                swap(board, row, col, row, col - 1);
                Some(rst)
            } else {
                None
            }
        }
        "RIGHT" => {
            if col < 2 {
                swap(board, row, col, row, col + 1);
                let rst = parse_string(board);
                swap(board, row, col, row, col + 1);
                Some(rst)
            } else {
                None
            }
        }
        _ => None
    }
}

/// 交换二者的位置
pub fn swap(board: &mut Vec<Vec<i32>>, r1: usize, c1: usize, r2: usize, c2: usize) {
    let tmp = board[r1][c1];
    board[r1][c1] = board[r2][c2];
    board[r2][c2] = tmp;
}

/// 把二维数组转为 字符串
pub fn parse_string(board: &Vec<Vec<i32>>) -> String {
    board.iter()
        .map(|o| -> String {
            o.iter().map(|o| o.to_string()).collect()
        })
        .collect()
}

/// 转字符串为二维数组
pub fn parse_vec(s: &String) -> Vec<Vec<i32>> {
    let char_arr: Vec<char> = s.chars().collect();
    let mut rst = vec![vec![0; 3]; 2];
    for i in 0..3 {
        rst[0][i] = (char_arr[i] as u8 - b'0') as i32;
    }
    for i in 3..6 {
        rst[1][i - 3] = (char_arr[i] as u8 - b'0') as i32;
    }
    rst
}