

pub fn compress_string(s: String) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.push('%');
    let mut ans = String::new();
    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            ans.push(chars[i - 1]);
            ans.push_str(count.to_string().as_str());
            count = 1;
        }
    }
    return if ans.len() < s.len() {
        ans
    } else { s }
}