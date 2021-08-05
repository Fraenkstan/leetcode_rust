use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    strs.into_iter().fold(HashMap::new(), |mut map, str| {
        let mut chars = str.chars().into_iter().collect::<Vec<char>>();
        chars.sort_unstable();
        map.entry(chars).or_insert(Vec::new()).push(str);
        map
    }).into_iter().map(|(_k, v)| v).collect::<Vec<Vec<String>>>()
}