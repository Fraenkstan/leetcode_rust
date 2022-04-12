pub fn number_of_steps(num: i32) -> i32 {
    let mut ans = 0;
    let mut num = num;
    while num > 0 {
        ans += 1;
        num = match num & 1 {
            0 => num >> 1,
            _ => num - 1,
        }
    }
    ans
}
