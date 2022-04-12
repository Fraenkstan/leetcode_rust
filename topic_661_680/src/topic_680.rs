

pub fn valid_palindrome(s: String) -> bool {
    let len = s.len();
    let vec = s.chars().collect::<Vec<char>>();
    cmp(&vec, 0, len - 1, 0)
}

fn cmp(vec: &Vec<char>, mut p_left: usize, mut p_right: usize, count: usize) -> bool {
    while p_left <= p_right {
        if vec[p_left] == vec[p_right] {
            p_left += 1;
            p_right -= 1;
        } else {
            if count == 1 {
                return false;
            }
            return cmp(vec, p_left + 1, p_right, count + 1) ||
                cmp(vec, p_left, p_right - 1, count + 1);
        }
    }
    true
}