pub fn permutation(s: String) -> Vec<String> {
    let mut chars = s.into_bytes();
    let mut ans = vec![];
    unsafe {
        dfs(&mut ans, &mut vec![], &mut chars);
    }
    ans
}

unsafe fn dfs(ans: &mut Vec<String>, path: &mut Vec<u8>, chars: &mut Vec<u8>) {
    if chars.is_empty() {
        ans.push(String::from_utf8_unchecked(path.clone()));
    }
    for i in 0..chars.len() {
        let c = chars.remove(i);
        path.push(c);
        dfs(ans, path, chars);
        chars.insert(i, c);
        path.pop();
    }
}
