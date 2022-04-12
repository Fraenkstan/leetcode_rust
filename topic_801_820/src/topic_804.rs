use std::collections::HashMap;

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let mut map = HashMap::new();
    words.iter().for_each(|word| {
        let mut key = String::new();
        for c in word.chars() {
            key.push_str(morse(c).as_str());
        }
        map.entry(key).or_insert(word);
    });
    map.keys().len() as i32
}

fn morse(c: char) -> String {
    match c {
        'a' => ".-".to_string(),
        'b' => "-...".to_string(),
        'c' => "-.-.".to_string(),
        'd' => "-..".to_string(),
        'e' => ".".to_string(),
        'f' => "..-.".to_string(),
        'g' => "--.".to_string(),
        'h' => "....".to_string(),
        'i' => "..".to_string(),
        'j' => ".---".to_string(),
        'k' => "-.-".to_string(),
        'l' => ".-..".to_string(),
        'm' => "--".to_string(),
        'n' => "-.".to_string(),
        'o' => "---".to_string(),
        'p' => ".--.".to_string(),
        'q' => "--.-".to_string(),
        'r' => ".-.".to_string(),
        's' => "...".to_string(),
        't' => "-".to_string(),
        'u' => "..-".to_string(),
        'v' => "...-".to_string(),
        'w' => ".--".to_string(),
        'x' => "-..-".to_string(),
        'y' => "-.--".to_string(),
        'z' => "--..".to_string(),
        _ => "".to_string(),
    }
}
