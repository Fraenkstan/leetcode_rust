pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    if envelopes.len() == 0 {
        return 0;
    }
    let mut envelopes = envelopes;
    envelopes.sort();
    let mut dp = vec![1; envelopes.len()];
    for i in 1..envelopes.len() {
        for j in 0..i {
            if envelopes[i][0] > envelopes[j][0] && envelopes[i][1] > envelopes[j][1] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    *dp.iter().max().unwrap()
}
