

pub fn remove_duplicates(s: String) -> String {
    let mut stack = Vec::new();
    for c in s.chars() {
        if let Some(ch) = stack.last() {
            if ch == &c {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }
    stack.iter().collect()
}