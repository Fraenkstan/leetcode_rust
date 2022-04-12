pub fn integer_break(n: i32) -> i32 {
    if n == 2 {
        return 1;
    }
    if n == 3 {
        return 2;
    }
    let mut dp = vec![0; n as usize + 1];
    dp[1] = 1;
    for i in 2..=n as usize {
        for j in 1..=i / 2 {
            dp[i] = dp[i].max(dp[i - j] * dp[j]).max(i as i32);
        }
    }
    dp[n as usize]
}
