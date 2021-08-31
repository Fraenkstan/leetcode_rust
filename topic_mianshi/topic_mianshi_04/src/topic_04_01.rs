use std::collections::HashSet;

pub fn find_whether_exists_path(n: i32, graph: Vec<Vec<i32>>, start: i32, target: i32) -> bool {
    let mut graph_map = vec![HashSet::new(); n as usize];
    graph.iter().for_each(|edge| {
        graph_map[edge[0] as usize].insert(edge[1]);
    });
    let mut visited = vec![false; n as usize];
    let mut queue = vec![];
    queue.push(start);
    while !queue.is_empty() {
        let from  = queue.remove(0);
        for to in graph_map[from as usize].iter() {
            if !visited[*to as usize] {
                if *to == target {
                    return true;
                }
                visited[*to as usize] = true;
                queue.push(*to);
            }
        }
    }
    false
}