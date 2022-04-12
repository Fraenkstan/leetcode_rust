use std::collections::HashSet;

pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut set = HashSet::new();
    for edge in edges {
        for node in edge {
            if set.contains(&node) {
                return node;
            }
            set.insert(node);
        }
    }
    0
}
