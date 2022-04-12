pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if mat.len() == 0 || mat[0].len() == 0 {
        return vec![];
    }
    let m = mat.len();
    let n = mat[0].len();
    let mut dp = vec![vec![i32::MAX - 1; n]; m];
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 0 {
                dp[i][j] = 0;
            } else {
                if i > 0 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j] + 1);
                }
                if j > 0 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i][j - 1] + 1);
                }
            }
        }
    }
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if dp[i][j] != 0 {
                if i < m - 1 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i + 1][j] + 1);
                }
                if j < n - 1 {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i][j + 1] + 1);
                }
            }
        }
    }
    dp
}
