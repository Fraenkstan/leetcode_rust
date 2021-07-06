use std::collections::HashSet;

pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut answer= HashSet::new();
    let mut candidates = candidates;
    candidates.sort_unstable();
    dfs(&mut Vec::new(), &candidates, 0, 0, target, &mut answer);
    answer.into_iter().collect()
}

fn dfs(path:&mut Vec<i32>, candidates:&Vec<i32>, idx:usize, sum:i32, target: i32, answer:&mut HashSet<Vec<i32>>) {
    if sum == target {
        answer.insert(path.clone());
        return;
    }
    for i in idx..candidates.len() {
        if sum + candidates[i] <= target {
            path.push(candidates[i]);
            dfs(path, candidates, i + 1, sum + candidates[i], target, answer);
            path.pop();
        }
    }
}