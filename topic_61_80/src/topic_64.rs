pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid.get(0).unwrap().len();
    let mut dp: Vec<i32> = vec![i32::max_value(); n + 1];
    dp[n - 1] = 0;
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            dp[j] = grid[i][j] + dp[j].min(dp[j + 1]);
        }
    }
    dp[0]
}
