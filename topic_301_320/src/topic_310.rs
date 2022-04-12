use std::mem::swap;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let (mut num_edges, mut graph, mut work,
        mut level) = (vec![0; n as usize], vec![Vec::new(); n as usize],
                      vec![0; n as usize], vec![0; n as usize]);
    let (mut leaf_cnt, mut num_vertices) = (0, n);
    edges.iter().for_each(|x| {
        num_edges[x[0] as usize] += 1;
        num_edges[x[1] as usize] += 1;
    });
    (0..n).into_iter().for_each(|i| graph[i as usize] = vec![0; num_edges[i as usize]]);
    edges.iter().for_each(|x| {
        graph[x[0] as usize][work[x[0] as usize] as usize] = x[1];
        work[x[0] as usize] += 1;
        graph[x[1] as usize][work[x[1] as usize] as usize] = x[0];
        work[x[1] as usize] += 1;
    });
    (0..n).into_iter().for_each(|i| if work[i as usize] == 1 {
        work[leaf_cnt as usize] = i;
        leaf_cnt += 1;
    });
    while num_vertices > 2 {
        let mut next_leaf_cnt = 0;
        for i in 0..leaf_cnt {
            let leaf = work[i];
            for index in &graph[leaf as usize] {
                num_edges[*index as usize] -= 1;
                if num_edges[*index as usize] == 1 {
                    level[next_leaf_cnt as usize] = *index;
                    next_leaf_cnt += 1;
                }
            }
        }
        num_vertices -= leaf_cnt as i32;
        leaf_cnt = next_leaf_cnt;
        swap(&mut work, &mut level)
    }

    return if leaf_cnt == 2 { vec![work[0], work[1]] } else { vec![work[0]] };
}