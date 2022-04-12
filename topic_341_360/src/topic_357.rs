

pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
    let mut dp = vec![(1, 1) ; n as usize + 1];
    dp[1] = (10, 9);
    for i in 2..=n as usize {
        if i < 10 {
            let tmp = dp[i - 1].1 * (10 - i as i32 + 1);
            dp[i] = (tmp + dp[i - 1].0, tmp);
        } else { dp[i] = dp[i - 1] }
    }
    dp[n as usize].0
}