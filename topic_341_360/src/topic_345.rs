pub fn reverse_vowels(s: String) -> String {
    let mut vec = s.chars().collect::<Vec<char>>();
    let (mut left, mut right) = (0 as usize, vec.len() - 1);
    while left < right {
        match (is_vowel(vec[left]), is_vowel(vec[right])) {
            (true, true) => {
                vec.swap(left, right);
                left += 1;
                right -= 1;
            }
            (true, false) => right -= 1,
            (false, true) => left += 1,
            (false, false) => {
                left += 1;
                right -= 1;
            }
        }
    }
    vec.into_iter().collect::<String>()
}

fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false,
    }
}
