pub fn arrange_coins(n: i32) -> i32 {
    ((0.25 + 2.0 * (n as f64)).sqrt() - 0.5) as i32
}
