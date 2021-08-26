


pub fn get_kth_magic_number(k: i32) -> i32 {
    let mut dp = vec![1; k as usize + 1];
    dp[0] = 1;
    let (mut a, mut b, mut c) = (0, 0, 0);
    for i in 1..k as usize {
        dp[i] = (dp[a] * 3).min(dp[b] * 5).min(dp[c] * 7);
        a += (dp[i] == dp[a] * 3) as usize;
        b += (dp[i] == dp[b] * 5) as usize;
        c += (dp[i] == dp[c] * 7) as usize;
    }
    dp[k as usize - 1]
}