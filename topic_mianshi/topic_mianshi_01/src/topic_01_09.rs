pub fn is_fliped_string(s1: String, s2: String) -> bool {
    s1.len() == s2.len() && s2.repeat(2).contains(&s1)
}
