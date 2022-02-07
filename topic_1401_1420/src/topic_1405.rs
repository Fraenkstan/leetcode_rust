
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut cnt = [(a, 'a'), (b, 'b'), (c, 'c')];
    let mut ans: Vec<char> = Vec::new();
    while {
        cnt.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let cur_len = ans.len();
        for (num, ch) in cnt.iter_mut() {
            if *num == 0
                || ans.len() >= 2 && *ch == *ans.last().unwrap() && *ch == ans[ans.len() - 2]
            {
                continue;
            }
            ans.push(*ch);
            *num -= 1;
            break;
        }
        ans.len() != cur_len
    } {}
    ans.iter().collect()
}