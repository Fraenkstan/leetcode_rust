pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let (n, k) = (n as usize, k as usize);
    let mut dp = vec![vec![0; n]; k + 1];
    dp[0][0] = 1;

    for i in 1..dp.len() {
        for edge in relation.iter() {
            let (src, dst) = (edge[0] as usize, edge[1] as usize);
            dp[i][dst] += dp[i - 1][src];
        }
    }
    return dp[k][n - 1];
}
