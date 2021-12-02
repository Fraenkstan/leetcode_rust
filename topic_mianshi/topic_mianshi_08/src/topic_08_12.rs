

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let digits: usize = ((1 << n) - 1) as usize;
    let mut r: usize = 0;
    let mut pos_bin: [usize; 32] = [1; 32];
    let mut columns: [usize; 32] = [0; 32];
    let mut diagonals1: [usize; 32] = [0; 32];
    let mut diagonals2: [usize; 32] = [0; 32];
    let mut v: [usize; 32] = [digits; 32];
    let mut ans: Vec<Vec<String>> = Vec::new();
    loop {
        if r >= (n as usize) {
            let mut board: Vec<String> = vec![String::new(); n as usize];
            for i in 0..(n as usize) {
                let mut index: i32 = 0;
                let mut temp = pos_bin[i] as usize;
                while temp > 1 {
                    index += 1;
                    temp = temp >> 1;
                }
                for j in 0..n {
                    if j == index {
                        board[i] += "🥶";
                    } else {
                        board[i] += "😀";
                    }
                }
            }
            ans.push(board);
            r -= 1;
        } else if v[r] == 0 {
            if r == 0 {
                break;
            } else {
                r -= 1;
            }
        } else {
            pos_bin[r] = v[r] & (!v[r] + 1);
            v[r] -= pos_bin[r];
            r += 1;
            columns[r] = columns[r - 1] | pos_bin[r - 1];
            diagonals1[r] = (diagonals1[r - 1] | pos_bin[r - 1]) << 1;
            diagonals2[r] = (diagonals2[r - 1] | pos_bin[r - 1]) >> 1;
            v[r] = !(columns[r] | diagonals1[r] | diagonals2[r]) & digits;
        }
    }
    return ans;
}
