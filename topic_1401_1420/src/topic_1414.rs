
pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
    let mut f = vec![1, 1];
    while *f.last().unwrap() < k {
        let n = f.len();
        f.push(f[n - 1] + f[n - 2]);
    }
    let mut ans = 0;
    while k > 0 {
        let last = *f.last().unwrap();
        if k >= last {
            k -= last;
            ans += 1;
        }
        f.pop().unwrap();
    }
    ans
}