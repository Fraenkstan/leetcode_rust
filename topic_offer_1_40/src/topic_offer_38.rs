
pub fn permutation(mut s: String) -> Vec<String> {
    let n = s.len();
    let mut ans = vec![];
    let mut t = String::with_capacity(n);
    let mut visited = vec![false; n];
    let chars = unsafe { s.as_bytes_mut() };
    chars.sort();
    dfs(0, &mut ans, &mut t, &mut visited, n, chars);
    ans
}

fn dfs(pos: usize, ans: &mut Vec<String>, t: &mut String, visited: &mut Vec<bool>, n: usize, s: &[u8]) {
    if pos == n {
        ans.push(t.clone());
        return;
    }
    (0..n).into_iter().for_each(|i| {
        if visited[i] || (i > 0 && s[i - 1] == s[i] && !visited[i - 1]) {
            return;
        }
        t.push(s[i] as char);
        visited[i] = true;
        dfs(pos + 1, ans, t, visited, n, s);
        t.pop();
        visited[i] = false;
    });
}

pub fn permutation_1(mut s: String) -> Vec<String> {
    let mut ans = vec![];
    let arr = unsafe { s.as_bytes_mut() };
    arr.sort();
    loop {
        let mut s = String::new();
        arr.iter().for_each(|c| {
            s.push(*c as char);
        });
        ans.push(s);
        if !next_permutation(arr) { break }
    }
    ans
}

fn reverse(arr: &mut [u8], start: usize) {
    let (mut left, mut right) = (start, arr.len() - 1);
    while left < right {
        arr.swap(left, right);
        left += 1;
        right -= 1;
    }
}

fn next_permutation(arr: &mut [u8]) -> bool {
    let mut i = arr.len() as i32 - 2;
    while i >= 0 && arr[i as usize] >= arr[i as usize + 1] {
        i -= 1;
    }
    if i < 0 {
        return false;
    }
    let mut j = arr.len() as i32 - 1;
    while j >= 0 && arr[i as usize] >= arr[j as usize] {
        j -= 1;
    }
    arr.swap(i as usize, j as usize);
    reverse(arr, i as usize + 1);
    true
}