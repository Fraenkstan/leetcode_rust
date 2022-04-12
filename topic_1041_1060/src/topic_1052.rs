pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let mut result = 0;
    let mut sum = 0;
    customers
        .iter()
        .zip(grumpy.iter())
        .for_each(|(c, g)| sum += c * (1 - g));
    let mut l = 0;
    for r in 0..customers.len() {
        // expand and update state
        sum += customers[r] * grumpy[r];
        // shrink and update state
        if r - l > x as usize - 1 {
            sum -= customers[l] * grumpy[l];
            l += 1;
        }
        // check update result
        result = result.max(sum)
    }
    result
}
