use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut map_ransom_node = HashMap::new();
    let mut map_magazine = HashMap::new();
    ransom_note.chars().into_iter().for_each(|c| {
        *map_ransom_node.entry(c).or_insert(0) += 1;
    });
    magazine.chars().into_iter().for_each(|c| {
        *map_magazine.entry(c).or_insert(0) += 1;
    });
    map_ransom_node.into_iter().all(|(c, count_ransom)| {
        let count_magazine = map_magazine.get(&c).unwrap_or_else(|| &0);
        count_ransom <= *count_magazine
    })
}
