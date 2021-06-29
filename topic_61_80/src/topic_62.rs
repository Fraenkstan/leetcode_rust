pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize]; m as usize];
    dp[0][0] = 1;
    (0..m as usize).into_iter().for_each(|i| {
        (0..n as usize).into_iter().for_each(|j| {
            if i > 0 && j > 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            } else if i > 0 {
                dp[i][j] = dp[i - 1][j];
            } else if j > 0 {
                dp[i][j] = dp[i][j - 1];
            }
        })
    });
    dp[m as usize - 1][n as usize - 1]
}
