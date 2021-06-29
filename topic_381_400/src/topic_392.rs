

pub fn is_subsequence(s: String, t: String) -> bool {
    let (mut i, mut j, iv, jv) = (0, 0, s.as_bytes(), t.as_bytes());
    while i < iv.len() && j < jv.len() {
        if iv[i] == jv[j] {
            i += 1;
        }
        j += 1;
    }
    i == iv.len()
}