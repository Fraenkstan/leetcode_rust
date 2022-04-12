pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![amount + 1; amount as usize + 1];
    dp[0] = 0;

    coins.into_iter().for_each(|coin| {
        (coin as usize..amount as usize + 1)
            .into_iter()
            .for_each(|i| dp[i] = dp[i].min(dp[(i - coin as usize)] + 1))
    });

    let res = if dp[amount as usize] > amount {
        -1
    } else {
        dp[amount as usize]
    };
    res
}
