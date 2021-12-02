

pub fn find_longest_word(s: String, mut dictionary: Vec<String>) -> String {
    dictionary.sort_by(|a, b|
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            b.len().cmp(&a.len())
        });
    let s = s.as_bytes();
    for c in dictionary.into_iter() {
        let mut i = 0;
        let mut res = true;
        for j in c.as_bytes() {
            while i < s.len() {
                if s[i] == *j {
                    break;
                }
                i += 1
            }
            if i == s.len() {
                res = false;
                break;
            }
            i += 1;
        }
        if res {
            return c
        }
    }
    "".to_string()
}