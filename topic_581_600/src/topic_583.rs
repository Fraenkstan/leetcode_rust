

pub fn min_distance(word1: String, word2: String) -> i32 {
    let mut dp = vec![vec![i32::max_value(); word2.len() + 1]; word1.len() + 1];
    for i in 0..=word1.len() {
        dp[i][0] = i as i32;
    }
    for j in 0..=word2.len() {
        dp[0][j] = j as i32;
    }
    for i in 1..=word1.len() {
        for j in 1..=word2.len() {
            if word1.chars().nth(i - 1).unwrap() == word2.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = std::cmp::min(dp[i][j - 1], dp[i - 1][j]) + 1;
            }
        }
    }
    dp[word1.len()][word2.len()]
}