pub fn smallest_good_base(n: String) -> String {
    let n_i64 = n.parse::<i64>().unwrap();
    for x in (2..=59).into_iter().rev() {
        let k = (n_i64 as f64).powf(1.0 / x as f64) as i64;
        if k <= 1 {
            continue;
        }
        let mut s = 0;
        for _i in (0..=x).into_iter() {
            s = s * k + 1;
        }
        if s == n_i64 {
            return k.to_string();
        }
    }
    (n_i64 - 1).to_string()
}
