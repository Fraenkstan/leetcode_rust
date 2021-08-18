

pub fn count_arrangement(n: i32) -> i32 {
    let mut dp = vec![0; 1 << n];
    dp[0] = 1;
    for mask in 1..(1 << n) {
        let num = i32::count_ones(mask) as i32;
        for i in 0..n {
            if (mask & (1 << i)) != 0 && ((num % (i + 1) == 0) || ((i + 1) % num == 0)) {
                dp[mask as usize] += dp[mask as usize ^ (1 << i)];
            }
        }
    }
    dp[(1 << n) as usize - 1]
}

pub fn count_arrangement_backstrace(n: i32) -> i32 {
    let mut vis = vec![false; (n + 1) as usize];
    let mut state = vec![Vec::new(); (n + 1) as usize];
    (1..=n).for_each(|i| (1..=n).for_each(|j| {
        if j % i == 0 || i %j == 0 {
            state[i as usize].push(j);
        }
    }));
    back_strace(1, n, &state, &mut vis)
}

fn back_strace(index: usize, n: i32, state: &Vec<Vec<i32>>, vis: &mut Vec<bool>) -> i32 {
    if index == (n + 1) as usize {
        return 1;
    }
    state[index].iter().fold(0, |mut ans, x| {
        let x = *x as usize;
        if !vis[x] {
            vis[x] = true;
            ans += back_strace(index + 1, n, state, vis);
            vis[x] = false;
        }
        ans
    })
}