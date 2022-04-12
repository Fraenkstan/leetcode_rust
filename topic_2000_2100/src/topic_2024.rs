pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    let (mut i, n, mut f_cnt, mut t_cnt, mut max_cnt, chars) = (
        0,
        answer_key.len(),
        0,
        0,
        0,
        answer_key.chars().collect::<Vec<char>>(),
    );
    for j in 0..n {
        max_cnt = if chars[j] == 'T' {
            t_cnt += 1;
            max_cnt.max(t_cnt)
        } else {
            f_cnt += 1;
            max_cnt.max(f_cnt)
        };
        if (j - i + 1) as i32 > max_cnt + k {
            if chars[i] == 'T' {
                t_cnt -= 1
            } else {
                f_cnt -= 1
            };
            i += 1;
        }
    }
    (n - i) as i32
}
