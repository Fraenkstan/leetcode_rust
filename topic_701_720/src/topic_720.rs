use std::collections::HashSet;

pub fn longest_word(mut words: Vec<String>) -> String {
    words.sort_unstable();
    let mut set = HashSet::new();
    let mut len = 1;
    let mut ans = "".to_string();
    for word in words.iter() {
        let prefix = word.split_at(word.len() - 1).0;
        if word.len() == 1 {
            set.insert(word.as_str());
            if ans == "" {
                ans = String::from(word);
            }
        }
        if set.contains(prefix) {
            set.insert(word.as_str());
            if word.len() > len {
                ans = String::from(word);
                len = word.len();
            }
        }
    }
    ans
}
