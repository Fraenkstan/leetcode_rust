

pub fn reverse_string(s: &mut Vec<char>) {
    let (mut left, mut right) = (0usize, s.len() - 1);
    while right > left {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}