pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut s1 = version1.split(".").map(|s| s.parse().unwrap());
    let mut s2 = version2.split(".").map(|s| s.parse().unwrap());
    loop {
        match (s1.next(), s2.next()) {
            (None, None) => break,
            (Some(0), None) | (None, Some(0)) => continue,
            (Some(_n1), None) => return 1,
            (None, Some(_n2)) => return -1,
            (Some(n1), Some(n2)) if n1 > n2 => return 1,
            (Some(n1), Some(n2)) if n1 < n2 => return -1,
            (Some(_n1), Some(_n2)) => continue,
        }
    }
    0
}
