pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut ans = Vec::<String>::new();
    for denominator in 2..=n {
        for numerator in 1..denominator {
            if gcd(numerator, denominator) == 1 {
                ans.push(format!("{}/{}", numerator, denominator));
            }
        }
    }
    ans
}

fn gcd(a: i32, b: i32) -> i32 {
    return if b != 0 { gcd(b, a % b) } else { a };
}
