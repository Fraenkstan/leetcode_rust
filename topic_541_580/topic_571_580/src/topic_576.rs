

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    use std::collections::HashMap;
    fn dfs(x: i32, y: i32, s: i32, m:i32, n:i32, max_move: i32, mem: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
        if s > max_move { return 0; }
        if x < 0 || y < 0 || x >= m || y >= n { return 1; }
        if let Some(&v) = mem.get(&(x, y, s)) {
            return v;
        }
        let mut ret = dfs(x + 1, y, s + 1, m, n, max_move, mem) as i64 +
            dfs(x - 1, y, s + 1, m, n, max_move, mem) as i64 +
            dfs(x, y + 1, s + 1, m, n, max_move, mem) as i64 +
            dfs(x, y - 1, s + 1, m, n, max_move, mem) as i64;
        ret = ret % 1000000007;
        let ret = ret as i32;
        mem.insert((x, y, s), ret);
        ret
    }

    dfs(start_row, start_column, 0, m , n, max_move, &mut HashMap::new())
}