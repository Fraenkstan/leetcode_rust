use std::collections::HashSet;

pub fn permutation(s: String) -> Vec<String> {
    let mut chars = s.into_bytes();
    let mut ans = vec![];
    let mut set = HashSet::new();
    unsafe {
        dfs(&mut ans, &mut vec![], &mut chars, &mut set);
    }
    ans
}

unsafe fn dfs(ans: &mut Vec<String>, path: &mut Vec<u8>, chars: &mut Vec<u8>, set: &mut HashSet<Vec<u8>>) {
    if chars.is_empty() {
        ans.push(String::from_utf8_unchecked(path.clone()));
    }
    for i in 0..chars.len() {
        let c = chars.remove(i);
        path.push(c);
        if !set.contains(path) {
            set.insert(path.clone());
            dfs(ans, path, chars, set);
        }
        chars.insert(i, c);
        path.pop();
    }
}