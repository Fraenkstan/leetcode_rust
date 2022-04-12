use std::vec::Vec;

fn is_invalid_parentheses(s: &str) -> bool {
    let mut count: u32 = 0;
    for c in s.chars() {
        if c == '(' {
            count = count + 1;
        } else if c == ')' {
            if count > 0 {
                count = count - 1;
            } else {
                return false;
            }
        }
    }

    count == 0
}

fn remove_invalid_parentheses_from_s(
    s: &str,
    start_index: usize,
    left_count: usize,
    right_count: usize,
    ret: &mut Vec<String>,
) {
    if left_count == 0 && right_count == 0 {
        if is_invalid_parentheses(s) {
            ret.push(String::from(s));
        }

        return;
    }

    let s_bytes = s.as_bytes();
    for i in start_index..s.len() {
        if i != start_index && s_bytes[i] == s_bytes[i - 1] {
            continue;
        }

        if left_count + right_count > s.len() - i {
            return;
        }

        let c = s_bytes[i];
        if c == '(' as u8 && left_count > 0 {
            let mut new_s = String::from(&s[..i]);
            new_s.push_str(&s[i + 1..]);
            remove_invalid_parentheses_from_s(
                &new_s,
                i,
                left_count - 1,
                right_count,
                ret,
            )
        }
        if c == ')' as u8 && right_count > 0 {
            let mut new_s = String::from(&s[..i]);
            new_s.push_str(&s[i + 1..]);
            remove_invalid_parentheses_from_s(
                &new_s,
                i,
                left_count,
                right_count - 1,
                ret,
            )
        }
    }
}

pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    // println!("{:?}", Solution::is_invalid_parentheses(&s));

    let mut left_count = 0;
    let mut right_count = 0;
    for c in s.chars() {
        if c == '(' {
            left_count = left_count + 1;
        } else if c == ')' {
            if left_count > 0 {
                left_count = left_count - 1;
            } else {
                right_count = right_count + 1;
            }
        }
    }

    // println!("{:?} {:?}", left_count, right_count);

    let mut ret: Vec<String> = Vec::new();
    remove_invalid_parentheses_from_s(&s, 0, left_count, right_count, &mut ret);
    ret
}
