pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid.get(0).unwrap().len();
    let mut dp = vec![vec![0; n as usize]; m as usize];
    dp[0][0] = 0;
    (0..m).into_iter().for_each(|i| {
        (0..n).into_iter().for_each(|j| {
            if obstacle_grid[i][j] == 1 {
                return;
            }
            if i > 0 && j > 0 {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            } else if i > 0 {
                dp[i][j] = dp[i - 1][j];
            } else if j > 0 {
                dp[i][j] = dp[i][j - 1];
            } else {
                dp[i][j] = 1
            }
        })
    });
    dp[m - 1][n - 1]
}
