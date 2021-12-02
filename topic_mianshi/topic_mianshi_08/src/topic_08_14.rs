

pub fn count_eval(s: String, result: i32) -> i32 {
    let str = s.into_bytes();
    let len = str.len();
    let mut dp = vec![vec![(0, 0); len / 2 + 1]; len / 2 + 1];

    for diff in 0..(len / 2 + 1) {
        let mut start = 0;
        while start + diff < len / 2 + 1 {
            let end = start + diff;
            if diff == 0 {
                if str[start * 2] == '1' as u8 {
                    dp[start][start].1 = 1;
                } else {
                    dp[start][start].0 = 1;
                }
            } else {
                for i in start..end {
                    let op = str[i * 2 + 1] as char;
                    caculate(op, &mut dp, start, end, i);
                }
            }
            start += 1;
        }
    }
    return if result == 1 {
        dp[0][len / 2].1
    } else {
        dp[0][len / 2].0
    };
}

fn caculate(op: char, dp: &mut Vec<Vec<(i32, i32)>>, start: usize, end: usize, i: usize) {
    match op {
        '&' => {
            dp[start][end].0 += dp[start][i].1 * dp[i + 1][end].0
                + dp[start][i].0 * dp[i + 1][end].0
                + dp[start][i].0 * dp[i + 1][end].1;
            dp[start][end].1 += dp[start][i].1 * dp[i + 1][end].1;
        }
        '|' => {
            dp[start][end].1 += dp[start][i].1 * dp[i + 1][end].0
                + dp[start][i].1 * dp[i + 1][end].1
                + dp[start][i].0 * dp[i + 1][end].1;
            dp[start][end].0 += dp[start][i].0 * dp[i + 1][end].0;
        }
        '^' => {
            dp[start][end].0 += dp[start][i].0 * dp[i + 1][end].0
                + dp[start][i].1 * dp[i + 1][end].1;
            dp[start][end].1 += dp[start][i].0 * dp[i + 1][end].1
                + dp[start][i].1 * dp[i + 1][end].0;
        }
        _ => {}
    }
}
