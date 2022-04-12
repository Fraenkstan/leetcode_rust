pub fn character_replacement(s: String, k: i32) -> i32 {
    let (mut begin, mut end, mut len, mut count) = (
        s.bytes().map(|x| x - b'A'),
        s.bytes().map(|x| x - b'A'),
        k,
        [0; 26],
    );

    if s.len() as i32 <= k - 1 {
        return s.len() as i32;
    }

    (0..k).for_each(|_| count[end.next().unwrap() as usize] += 1);

    while let Some(x) = end.next() {
        count[x as usize] += 1;
        if *count.iter().max().unwrap() > len - k {
            len += 1
        } else {
            count[begin.next().unwrap() as usize] -= 1
        }
    }
    len
}
