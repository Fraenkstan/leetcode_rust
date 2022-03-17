
pub fn reverse_only_letters(s: String) -> String {
    if s.len() == 0 { return "".to_string(); }
    let mut cv: Vec<char> = s.chars().collect();
    let mut p = 0;
    let mut q = s.len() - 1;
    while p < q {
        if cv[p].is_alphabetic() && cv[q].is_alphabetic() {
            cv.swap(p, q);
            p += 1;
            q -= 1;
        } else if cv[p].is_alphabetic() {
            q -= 1;
        } else if cv[q].is_alphabetic() {
            p += 1;
        } else {
            p += 1;
            q -= 1;
        }
    }
    cv.into_iter().collect()
}