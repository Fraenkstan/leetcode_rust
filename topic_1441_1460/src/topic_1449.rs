
pub fn largest_number(cost: Vec<i32>, target: i32) -> String {

    let mut dp = vec![i32::MIN; target as usize + 1];
    dp[0] = 0;

    cost.iter().for_each(|c| {
        for j in *c as usize..=target as usize {
            dp[j] = dp[j].max(dp[j - *c as usize] + 1);
        }
    });

    if dp[target as usize] < 0 {
        return "0".to_string();
    }

    let mut ans = String::new();
    let mut j = target as usize;
    for i in (0..=8).rev() {
        let c = cost[i] as usize;
        while j >= c && dp[j] == dp[j - c] + 1 {
            ans.push_str(&((1 + i).to_string()));
            j -= c;
        }
    }
    ans
}