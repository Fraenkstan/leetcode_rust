pub fn exchange_bits(mut num: i32) -> i32 {
    let mut ans = 0;
    let mut count = 0;
    while num > 0 {
        let mut last = (num & 1);
        num >>= 1;
        if count % 2 == 0 {
            last <<= 1 * (count + 1);
        } else {
            last <<= 1 * (count - 1);
        }
        ans |= last;
        count += 1;
    }
    ans
}
