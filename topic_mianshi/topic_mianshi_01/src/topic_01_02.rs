use std::collections::BTreeMap;

pub fn check_permutation(s1: String, s2: String) -> bool {
    let map1 = to_map(s1);
    let map2 = to_map(s2);
    if map1.len() != map2.len() {
        return false;
    }
    map1.iter()
        .zip(map2.iter())
        .all(|((k1, v1), (k2, v2))| k1 == k2 && v1 == v2)
}

fn to_map(s: String) -> BTreeMap<char, i32> {
    s.chars().fold(BTreeMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    })
}
