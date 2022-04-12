use std::collections::VecDeque;

pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let mut queue = VecDeque::with_capacity(n);
    let mut seen = vec![vec![false; 1 << n]; n];
    for i in 0..n {
        queue.push_front((i, 1 << i, 0));
        seen[i][1 << i] = true;
    }
    let mut ans = 0;
    while !queue.is_empty() {
        let tuple = queue.pop_back().unwrap();
        let (u, mask, dist) = (tuple.0, tuple.1, tuple.2);
        if mask == ((1 << n) - 1) {
            ans = dist;
            break;
        }
        graph[u].iter().for_each(|&v| {
            let v = v as usize;
            let mask_v = mask | (1 << v);
            if !seen[v][mask_v] {
                queue.push_front((v, mask_v, dist + 1));
                seen[v][mask_v] = true;
            }
        });
    }
    ans
}
