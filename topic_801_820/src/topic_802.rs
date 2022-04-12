pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let mut color = vec![0; graph.len()];
    (0..graph.len())
        .into_iter()
        .filter(|&i| safe(&graph, &mut color, i))
        .map(|i| i as i32)
        .collect()
}

fn safe(graph: &Vec<Vec<i32>>, color: &mut Vec<i32>, x: usize) -> bool {
    if color[x] > 0 {
        return color[x] == 2;
    }
    color[x] = 1;
    if graph[x].iter().any(|&y| !safe(graph, color, y as usize)) {
        return false;
    }
    color[x] = 2;
    true
}
