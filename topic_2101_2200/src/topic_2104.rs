pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
    let mut ans = 0;
    let mut max_dp = vec![vec![0; 1001]; 1001];
    let mut min_dp = vec![vec![0; 1001]; 1001];
    for (i, &n) in nums.iter().enumerate() {
        max_dp[i][i] = n;
        min_dp[i][i] = n;
    }
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            max_dp[i][j] = max_dp[i][j - 1].max(nums[j]);
            min_dp[i][j] = min_dp[i][j - 1].min(nums[j]);
            ans += (max_dp[i][j] - min_dp[i][j]) as i64;
        }
    }
    ans
}
