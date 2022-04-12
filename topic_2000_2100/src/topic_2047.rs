pub fn count_valid_words(sentence: String) -> i32 {
    let words = sentence.split_whitespace().collect::<Vec<_>>();
    words
        .iter()
        .fold(0, |acc, &word| acc + if is_valid(word) { 1 } else { 0 })
}

fn is_valid(word: &str) -> bool {
    let len = word.len();
    let chars = word.chars().collect::<Vec<char>>();
    let mut flag = false;
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            return false;
        }
        match c {
            '-' => {
                if flag
                    || i == 0
                    || i == len - 1
                    || !chars[i - 1].is_alphabetic()
                    || !chars[i + 1].is_alphabetic()
                {
                    return false;
                }
                flag = true;
            }
            '!' | '.' | ',' => {
                if i != len - 1 {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}
