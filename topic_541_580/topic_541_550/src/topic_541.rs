

pub fn reverse_str(s: String, k: i32) -> String {
    s.into_bytes().chunks_mut(2 * k as usize).map(|ch| {
        if ch.len() < k as usize {
            ch.reverse();
        } else {
            ch[..k as usize].reverse();
        }
        String::from_utf8_lossy(ch)
    }).collect::<String>()
}