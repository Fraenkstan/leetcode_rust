pub fn merge(a: &mut Vec<i32>, mut m: i32, b: &mut Vec<i32>, n: i32) {
    for i in 0..n as usize {
        let mut p = a.len() - 1;
        a[p] = b[i];
        while p > 0 && (a[p - 1] > a[p] || p > m as usize) {
            a.swap(p - 1, p);
            p -= 1;
        }
        m += 1;
    }
}
