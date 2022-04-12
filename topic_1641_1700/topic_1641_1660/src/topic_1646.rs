pub fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let n = n as usize;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 1..=(n / 2) {
        dp[2 * i] = dp[i];
        if 2 * i + 1 <= n {
            dp[2 * i + 1] = dp[i] + dp[i + 1];
        }
    }
    dp.into_iter().max().unwrap()
}
