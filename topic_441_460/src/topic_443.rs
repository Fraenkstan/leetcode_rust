pub fn compress(chars: &mut Vec<char>) -> i32 {
    let len = chars.len();
    let mut ans = 0;
    let mut p1 = 0;
    while p1 < len {
        let mut p2 = p1;
        while p2 < len && chars[p1] == chars[p2] {
            p2 += 1;
        }
        chars[ans] = chars[p1];
        ans += 1;
        if p2 - 1 != p1 {
            (p2 - p1).to_string().chars().into_iter().for_each(|c| {
                chars[ans] = c;
                ans += 1;
            });
        }
        p1 = p2;
    }
    ans as i32
}
