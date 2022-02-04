

pub fn reverse_prefix(word: String, ch: char) -> String {
    if let Some(i) = word.find(ch) {
        word[..=i].chars().rev().collect::<String>() + &word[i + 1..]
    } else {
        word
    }
}