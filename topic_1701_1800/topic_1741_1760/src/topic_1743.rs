use std::collections::{HashMap, HashSet};

pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut freq = HashMap::new();
    for v in adjacent_pairs.iter() {
        map.entry(v[0]).or_insert(Vec::new()).push(v[1]);
        map.entry(v[1]).or_insert(Vec::new()).push(v[0]);
        *freq.entry(v[0]).or_insert(0) += 1;
        *freq.entry(v[1]).or_insert(0) += 1;
    }
    let start = *freq.iter().find(|x| *x.1 == 1).unwrap().0;
    let mut answer = Vec::new();

    let mut visited = HashSet::new();
    let mut stack = vec![start];
    while !stack.is_empty() {
        let s = stack.pop().unwrap();
        if visited.insert(s) {
            answer.push(s);
            if let Some(neighbors) = map.get(&s) {
                for &neighbor in neighbors {
                    stack.push(neighbor);
                }
            }
        }
    }
    answer
}
