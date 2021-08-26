

pub fn reverse_words(s: String) -> String {
    let mut ans = s.split(" ").into_iter().fold(String::new(), |mut ans, str| {
        ans.push_str(str.chars().into_iter().rev().collect::<String>().as_str());
        ans.push(' ');
        ans
    }).to_string();
    ans.remove(ans.len() - 1);
    ans
}