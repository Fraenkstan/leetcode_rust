
pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
    fn dfs(
        graph: &mut Vec<Vec<i32>>,
        in_degree: &mut Vec<i32>,
        cur: i32,
        n: i32,
        res: &mut Vec<i32>,
    ) {
        if cur < n {
            res.push(cur);
        }
        in_degree[cur as usize] -= 1;

        for &child in &graph[cur as usize].clone() {
            in_degree[child as usize] -= 1;
            if in_degree[child as usize] == 0 {
                dfs(graph, in_degree, child, n, res);
            }
        }
    }

    let mut ans = vec![];
    let mut graph = vec![vec![]; (n + m) as usize];
    let mut in_degree = vec![0; (n + m) as usize];

    for i in 0..group.len() {
        if group[i] == -1 {
            continue;
        }
        graph[(n + group[i]) as usize].push(i as i32);
        in_degree[i] += 1;
    }

    for i in 0..before_items.len() {
        for &item in &before_items[i] {
            let rep_before_item = if group[item as usize] == -1 {
                item
            } else {
                n + group[item as usize]
            };
            let rep_current_item = if group[i] == -1 {
                i as i32
            } else {
                n + group[i]
            };

            if rep_before_item == rep_current_item {
                graph[item as usize].push(i as i32);
                in_degree[i] += 1;
            } else {
                graph[rep_before_item as usize].push(rep_current_item);
                in_degree[rep_current_item as usize] += 1;
            }
        }
    }

    for i in 0..n + m {
        if in_degree[i as usize] == 0 {
            dfs(&mut graph, &mut in_degree, i, n, &mut ans);
        }
    }
    if ans.len() as i32 == n {
        ans
    } else {
        vec![]
    }
}