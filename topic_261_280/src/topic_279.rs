pub fn num_squares(n: i32) -> i32 {
    let mut dp = vec![n; n as usize + 1];
    dp[0] = 0;

    let mut result = n;
    (0..=n as usize).into_iter().for_each(|i| {
        let x = i * i;
        (x..=n as usize).into_iter().for_each(|j| {
            dp[j] = dp[j].min(dp[j - x] + 1);
        });
        result = result.min(dp[n as usize]);
    });
    result
}
