
unsafe fn guess_number(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;
    while l < r {
        let mid = l + (r - l) / 2;
        match guess(mid) {
            -1 => l = mid + 1,
            1 => r = mid,
            0 => return mid,
            _ => ()
        }
    }
    l
}

unsafe fn guess(_num: i32) -> i32 {
    1
}