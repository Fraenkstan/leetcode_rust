pub fn nth_ugly_number(n: i32) -> i32 {
    let mut dp = vec![0; n as usize + 1];
    dp[1] = 1;
    let mut p2 = 1;
    let mut p3 = 1;
    let mut p5 = 1;
    (2..=n).into_iter().for_each(|i| {
        let num2 = dp[p2] * 2;
        let num3 = dp[p3] * 3;
        let num5 = dp[p5] * 5;
        dp[i as usize] = num2.min(num3).min(num5);
        if dp[i as usize] == num2 {
            p2 += 1
        }
        if dp[i as usize] == num3 {
            p3 += 1
        }
        if dp[i as usize] == num5 {
            p5 += 1
        }
    });
    dp[n as usize]
}
