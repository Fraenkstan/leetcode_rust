
pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut ans = 0;
    for n in left..=right {
        match n.count_ones() {
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 27 | 29 | 31 | 32 => ans += 1,
            _ => (),
        }
    }
    ans
}