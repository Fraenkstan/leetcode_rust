pub fn first_bad_version(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;
    while l < r {
        let mid = l + (r - l) / 2;
        if is_bad_version(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn is_bad_version(_versions: i32) -> bool {
    true
}
