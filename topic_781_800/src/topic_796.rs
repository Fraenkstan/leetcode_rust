use std::collections::HashSet;

pub fn rotate_string(mut s: String, goal: String) -> bool {
    let mut set = HashSet::new();
    while !set.contains(&s) {
        set.insert(s.clone());
        s = s.chars().into_iter().skip(1).chain(s.chars().into_iter().take(1)).collect::<String>();
    }
    set.contains(&goal)
}