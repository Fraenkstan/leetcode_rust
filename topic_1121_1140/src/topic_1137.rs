

pub fn tribonacci(n: i32) -> i32 {
    (0..n).fold((0, 1, 1), |t, _| (t.1, t.2, t.0 + t.1 + t.2)).0
}