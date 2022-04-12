pub fn check_valid_string(s: String) -> bool {
    let chars = s.chars().collect::<Vec<char>>();
    let mut vec = vec![];
    for i in 0..chars.len() {
        match chars[i] {
            '(' => {
                vec.push('(');
            }
            ')' => {
                if vec.len() == 0 {
                    return false;
                } else if vec.contains(&'(') {
                    for i in (0..vec.len()).rev() {
                        if vec[i] == '(' {
                            vec.remove(i);
                            break;
                        }
                    }
                } else {
                    vec.pop();
                }
            }
            '*' => {
                vec.push('*');
            }
            _ => {}
        }
    }
    vec.iter().fold(0, |mut acc, &i| {
        if i == '(' {
            acc += 1;
        } else {
            acc = 0.max(acc - 1);
        }
        acc
    }) == 0
}
