

pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
    fn dfs(root: &Vec<(usize, usize, usize)>, cur: usize, ans: &mut i32, max: &mut usize) -> usize {
        if cur >= root.len() { return 0; }
        let l_cnt = dfs(root, root[cur].1, ans, max);
        let r_cnt = dfs(root, root[cur].2, ans, max);
        let tmp = if l_cnt == 0 { 1 } else { l_cnt } *
            if r_cnt == 0 { 1 } else { r_cnt } *
            if root.len() - l_cnt - r_cnt - 1 == 0 { 1 } else { root.len() - l_cnt - r_cnt - 1 };
        if tmp > *max {
            *max = tmp;
            *ans = 1;
        } else if tmp == *max {
            *ans += 1;
        }
        l_cnt + r_cnt + 1
    }
    let mut ans = 0;
    let mut max = 0;
    let n = parents.len();
    let mut root = vec![(n, n, n); n];
    for (i, p) in parents.into_iter().enumerate() {
        if p == -1 { continue; }
        if root[p as usize].1 == n {
            root[p as usize].1 = i;
        } else {
            root[p as usize].2 = i;
        }
        root[i].0 = p as usize;
    }
    dfs(&root, 0, &mut ans, &mut max);
    ans
}