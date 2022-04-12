pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let k = primes.len();
    let mut dp = Vec::with_capacity(n as usize);
    dp.push(1);
    let mut index = vec![0; k];
    for _ in 1..n as usize {
        let mut min = i32::MAX;
        for i in 0..k {
            min = min.min(primes[i] * dp[index[i]]);
        }
        dp.push(min);
        for i in 0..k {
            if primes[i] * dp[index[i]] == min {
                index[i] += 1;
            }
        }
    }
    dp[n as usize - 1]
}
