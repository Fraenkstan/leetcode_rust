pub fn find_integers(n: i32) -> i32 {
    let n = n + 1;
    let b: Vec<_> = format!("{:b}", n).chars().rev().collect();
    let n = b.len();
    let dp: Vec<_> = (0..n)
        .scan((1, 0), |s, _| {
            *s = (s.0 + s.1, s.0);
            Some(*s)
        })
        .collect();
    let mut ans = 0;
    for i in (0..n).rev() {
        if b[i] == '1' {
            ans += dp[i].0;
        }
        if i < b.len() - 1 && b[i] == '1' && b[i + 1] == '1' {
            break;
        }
    }
    ans
}
