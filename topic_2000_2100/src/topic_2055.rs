pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, _n) = (s.len(), queries.len());
    let s_chars = s.chars().collect::<Vec<_>>();
    let arr = (0..m).fold(
        (vec![0; m + 1], vec![i32::MAX; m + 1], vec![0; m + 1]),
        |(mut prev, mut next, mut pre_sum), i| {
            pre_sum[i + 1] = if s_chars[i] == '|' {
                pre_sum[i] + 1
            } else {
                pre_sum[i]
            };
            prev[i + 1] = if s_chars[i] == '|' { i as i32 } else { prev[i] };
            next[m - i - 1] = if s_chars[m - i - 1] == '|' {
                (m - i - 1) as i32
            } else {
                next[m - i]
            };
            (prev, next, pre_sum)
        },
    );

    // println!("{:?}-{:?}-{:?}", arr.0, arr.1, arr.2);
    queries.iter().fold(Vec::new(), |mut ret, query| {
        let (l, r) = (arr.1[query[0] as usize], arr.0[query[1] as usize + 1]);
        ret.push(if l < r {
            r - l - (arr.2[r as usize] - arr.2[l as usize])
        } else {
            0
        });
        ret
    })
}
