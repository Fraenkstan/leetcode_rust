

pub fn print_bin(mut num: f64) -> String {
    let mut ans = "0.".to_string();
    while num != 0_f64 {
        num *= 2_f64;
        if num >= 1_f64 {
            ans.push('1');
            num -= 1_f64;
        } else {
            ans.push('0');
        }
        if ans.len() > 32 {
            return "ERROR".to_string();
        }
    }
    ans
}