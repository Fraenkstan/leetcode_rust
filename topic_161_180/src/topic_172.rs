


pub fn trailing_zeroes(n: i32) -> i32 {
    let (mut n, mut ans) = (n, 0);
    while n > 0 {
        n /= 5;
        ans += n;
    }
    ans
}