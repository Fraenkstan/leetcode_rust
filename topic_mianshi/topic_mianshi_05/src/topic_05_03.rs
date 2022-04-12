pub fn reverse_bits(num: i32) -> i32 {
    let mut ans = 0;
    let mut pre_bits = 0;
    let mut cur_bits = 0;
    if num == -1 {
        return 32;
    }
    for i in 0..32 {
        if (num >> i) & 1 == 1 {
            cur_bits += 1;
        } else {
            pre_bits = cur_bits;
            cur_bits = 0;
        }
        ans = ans.max(cur_bits + 1 + pre_bits);
    }
    return ans;
}
