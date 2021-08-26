use std::collections::VecDeque;

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut queue = VecDeque::new();
    queue.push_back(vec![0]);
    let mut result = vec![];
    loop {
        if let Some(path) = queue.pop_front() {
            let end = path[path.len()-1];
            if end == (graph.len()-1) as i32 {
                result.push(path);
            } else {
                for next in &graph[end as usize] {
                    let mut path1 = path.clone();
                    path1.push(*next);
                    queue.push_back(path1);
                }
            }
        } else {
            break;
        }
    }
    result
}