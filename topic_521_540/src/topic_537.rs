pub fn complex_number_multiply(num1: String, num2: String) -> String {
    fn str_to_complex(s: String) -> (i32, i32) {
        let idx = s.find('+').unwrap();
        (
            (&s[0..idx]).parse().unwrap(),
            (&s[idx + 1..s.len() - 1]).parse().unwrap(),
        )
    }
    let x = str_to_complex(num1);
    let y = str_to_complex(num2);
    format!("{}+{}i", x.0 * y.0 - x.1 * y.1, x.0 * y.1 + x.1 * y.0)
}
