use std::collections::HashMap;

pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
    let mut weights = HashMap::new();
    for (i, &n) in target.iter().enumerate() {
        weights.entry(n).or_insert(i);
    }
    let mut queue = vec![];
    for n in arr.iter() {
        if let Some(&w) = weights.get(n) {
            if queue.is_empty() {
                queue.push(w);
            } else {
                match queue.binary_search(&w) {
                    Err(index) => {
                        if index == queue.len() {
                            queue.push(w);
                        } else {
                            queue[index] = w;
                        }
                    }
                    _ => {}
                };
            }
        }
    }
    return target.len() as i32 - queue.len() as i32;
}
