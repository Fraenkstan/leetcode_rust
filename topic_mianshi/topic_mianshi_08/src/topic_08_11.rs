

pub fn ways_to_change(n: i32) -> i32 {
    let mut dp = vec![0; n.max(25) as usize + 1];
    dp[0] = 1;
    for &mask in [1usize, 5, 10, 25].iter() {
        for i in mask..=n as usize {
            dp[i] = (dp[i] + dp[i - mask]) % 1000000007;
        }
    }
    dp[n as usize]
}