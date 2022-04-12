pub fn ways_to_step(n: i32) -> i32 {
    let n = n as usize;
    let len = n.max(3) + 1;
    let mut dp = vec![0; len];
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 4;
    for i in 4..=n {
        dp[i] = ((dp[i - 3] + dp[i - 2]) % 1000000007 + dp[i - 1]) % 1000000007;
    }
    dp[n]
}
