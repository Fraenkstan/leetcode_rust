
pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; k as usize + 1];
    let k = k as usize;
    (0..=k).for_each(|step| {
        (0..n).for_each(|i| {
            (0..n).for_each(|j| {
                if step == 0 {
                    dp[step][i as usize][j as usize] = 1.0;
                } else {
                    for (dx, dy) in [(-2, -1), (-2, 1), (2, -1), (2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2)] {
                        let ni = i + dx;
                        let nj = j + dy;
                        if ni >= 0 && ni < n && nj >= 0 && nj < n {
                            let i = i as usize;
                            let j = j as usize;
                            let ni = ni as usize;
                            let nj = nj as usize;
                            dp[step][i][j] += dp[step - 1][ni][nj] / 8.0;
                        }
                    }
                }
            })
        })
    });
    dp[k][row as usize][column as usize]
}