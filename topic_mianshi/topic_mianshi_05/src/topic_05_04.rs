

pub fn find_closed_numbers(num: i32) -> Vec<i32> {
    if num <= 0 {
        return vec![-1, -1];
    }
    vec![get_next(num), get_prev(num)]
}

fn get_next(num: i32) -> i32 {
    let mut num = num;
    let mut c = num;
    let (mut c0, mut c1) = (0, 0);
    while c & 1 == 0 && c != 0 {
        c0 += 1;
        c = c >> 1;
    }
    while c & 1 == 1 {
        c1 += 1;
        c = c >> 1;
    }
    if c0 + c1 == 31 || c0 + c1 == 0 {
        return -1;
    }
    let p = c0 + c1;
    num = num | (1 << p);
    num = num & !((1 << p) - 1);
    num = num | (1 << (c1 - 1)) - 1;
    num
}

fn get_prev(num: i32) -> i32 {
    let mut num = num;
    let mut temp = num;
    let (mut c0, mut c1) = (0, 0);
    while temp & 1 == 1 {
        c1 += 1;
        temp = temp >> 1;
    }
    if temp == 0 {
        return -1;
    }
    while temp & 1 == 0 && temp != 0 {
        c0 += 1;
        temp = temp >> 1;
    }
    let p = c0 + c1;
    num = num & (!0 << (p + 1));
    let mask = (1 << (c1 + 1)) -1;
    num = num | mask << (c0 - 1);
    num
}

#[test]
fn test() {
    println!("next: {}, prev: {}", get_next(2), get_prev(2));
    println!("next: {}, prev: {}", get_next(1), get_prev(1));
}