pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    const MAX: i32 = 10000 * 101 + 1;
    let mut dp = vec![vec![MAX; n as usize]; k as usize + 2];
    dp[0][src as usize] = 0;
    for i in 1..k as usize + 2 {
        for flight in &flights {
            dp[i][flight[1] as usize] =
                dp[i][flight[1] as usize].min(dp[i - 1][flight[0] as usize] + flight[2]);
        }
    }
    let mut ans = MAX;
    for i in 1..k as usize + 2 {
        ans = ans.min(dp[i][dst as usize])
    }
    return if ans == MAX { -1 } else { ans };
}
