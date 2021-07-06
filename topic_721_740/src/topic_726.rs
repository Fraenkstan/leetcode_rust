use std::collections::HashMap;

pub fn count_of_atoms(formula: String) -> String {
    let bytes = formula.as_bytes();
    let mut stack: Vec<HashMap<String, i32>> = Vec::new();
    let mut i = 0;
    let n = formula.len();
    stack.push(HashMap::new());
    while i < n {
        let c = bytes[i] as char;
        match c {
            '(' => {
                stack.push(HashMap::new());
                i += 1;
            }
            ')' => {
                let top = stack.pop().unwrap();
                i += 1;
                let start = i;
                while i < n && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                let number = toi32(&bytes[start..i]);
                let m: &mut HashMap<String, i32> = stack.last_mut().unwrap();
                for (name, count) in top {
                    *m.entry(name).or_insert(0) += count * number;
                }
            },
            _ => {
                // parse atom name
                let start = i + 1;
                i += 1;
                while i < n && bytes[i].is_ascii_lowercase() {
                    i += 1;
                }
                let name = to_string(&bytes[start - 1..i]);

                // parse atom count
                let start = i;
                while i < n && bytes[i].is_ascii_digit() {
                    i += 1;
                }
                let count = toi32(&bytes[start..i]);
                let m: &mut HashMap<String, i32> = stack.last_mut().unwrap();
                *m.entry(name).or_insert(0) += count;
            }
        }
    }

    let map = stack.last().unwrap();
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));

    let mut res = "".to_string();
    for (name, count) in v {
        res.push_str(name);
        if *count > 1 {
            res.push_str(&count.to_string());
        }
    }
    res.to_owned()
}

fn toi32(v: &[u8]) -> i32 {
    if v.len() == 0 {
        return 1;
    }
    std::str::FromStr::from_str(std::str::from_utf8(v).unwrap()).unwrap()
}

fn to_string(v: &[u8]) -> String {
    std::str::from_utf8(v).unwrap().to_string()
}
