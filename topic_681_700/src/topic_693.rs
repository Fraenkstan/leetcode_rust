pub fn has_alternating_bits(n: i32) -> bool {
    let mut n = n;
    let mut prv = 1 - n % 2;
    while n > 0 {
        if prv == n % 2 {
            return false;
        }
        prv = n % 2;
        n /= 2;
    }
    true
}
