pub fn remove_kdigits(num: String, mut k: i32) -> String {
    let mut v = vec![];
    for c in num.chars() {
        while let Some(&x) = v.last() {
            if k == 0 || x <= c {
                break;
            }
            v.pop();
            k -= 1;
        }
        v.push(c);
    }
    while k > 0 {
        v.pop();
        k -= 1;
    }
    let s: String = v.into_iter().skip_while(|&c| c == '0').collect();
    if s.is_empty() {
        "0".to_string()
    } else {
        s
    }
}
