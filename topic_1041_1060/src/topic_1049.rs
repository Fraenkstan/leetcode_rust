pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let length = stones.len();
    if length == 0 {
        return 0;
    }
    let sum = stones.iter().sum::<i32>();
    let max_weight = sum / 2;
    let mut dp = Vec::<bool>::with_capacity(max_weight as usize + 1);
    dp.push(true);
    for _i in 2..=dp.capacity() {
        dp.push(false);
    }
    for stone in stones {
        let mut i = max_weight as usize;
        while i >= stone as usize {
            dp[i] = dp[i] || dp[i - stone as usize];
            i -= 1;
        }
    }
    let mut i = max_weight as usize;
    loop {
        if dp[i] {
            return sum - 2 * (i as i32);
        }
        i -= 1;
    }
}

pub fn solulast_stone_weight_ii_2(stones: Vec<i32>) -> i32 {
    let sum = stones.iter().sum::<i32>();
    let max_weight = (sum / 2) as usize;
    let mut dp = vec![0; max_weight + 1];
    for stone in stones {
        for b in (stone..=max_weight as i32).rev() {
            dp[b as usize] = dp[b as usize].max(dp[(b - stone) as usize] + stone);
        }
    }
    sum - dp[max_weight] * 2
}
