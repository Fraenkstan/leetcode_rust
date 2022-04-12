pub fn longest_nice_substring(s: String) -> String {
    let n = s.len();
    let mut max_pos = 0;
    let mut max_len = 0;
    for i in 0..n {
        let (mut lower, mut upper) = (0, 0);
        for j in i..n {
            let c = s.chars().nth(j).unwrap();
            if c.is_lowercase() {
                lower |= 1 << (c as i32 - 'a' as i32);
            } else {
                upper |= 1 << (c as i32 - 'A' as i32);
            }
            if lower == upper && j - i + 1 > max_len {
                max_pos = i;
                max_len = j - i + 1;
            }
        }
    }
    s[max_pos..max_pos + max_len].to_owned()
}
