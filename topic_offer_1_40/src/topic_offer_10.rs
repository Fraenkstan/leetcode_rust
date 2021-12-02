

pub fn fib(n: i32) -> i32 {
    let mut dp = vec![0; (n as usize).max(2) + 1];
    dp[1] = 1;
    for i in 2..=n as usize {
        dp[i] = (dp[i - 2] + dp[i - 1]) % 1000000007;
    }
    dp[n as usize]
}