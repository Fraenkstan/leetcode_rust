

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut ans = vec![];
    dfs(&mut ans, &mut String::new(), 0, n, n, n);
    ans
}

fn dfs(ans: &mut Vec<String>, path: &mut String, step: i32, n: i32, left: i32, right: i32) {
    if step == n * 2 {
        ans.push(path.clone());
        return;
    }
    if left > 0 {
        path.push('(');
        dfs(ans, path, step + 1, n, left - 1, right);
        path.pop();
    }
    if right > left {
        path.push(')');
        dfs(ans, path, step + 1, n, left, right - 1);
        path.pop();
    }
}