pub fn multiply(a: i32, b: i32) -> i32 {
    if b == 0 {
        return 0;
    }
    a + multiply(a, b - 1)
}
