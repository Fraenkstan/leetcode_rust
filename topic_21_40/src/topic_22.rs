fn back_trace(s: String, n: usize, right: usize, left: usize, result: &mut Vec<String>) {
    if s.len() == 2 * n {
        result.push(s);
        return;
    }

    if left < n {
        let mut s = s.to_owned();
        s.push('(');
        back_trace(s, n, right, left + 1, result);
    }

    if left > 0 && left > right && right < n {
        let mut s = s.to_owned();
        s.push(')');
        back_trace(s, n, right + 1, left, result);
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result = vec![];
    back_trace(String::new(), n as usize, 0, 0, &mut result);
    result
}
