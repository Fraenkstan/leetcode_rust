use std::collections::HashMap;

pub fn can_permute_palindrome(s: String) -> bool {
    let map = s.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    let n = map.into_iter().filter(|(_k, v)| v % 2 != 0).count();
    n <= 1
}
