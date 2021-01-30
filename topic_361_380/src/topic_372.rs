pub fn super_pow(a: i32, mut b: Vec<i32>) -> i32 {
    let a = a % 1337;
    let mut ret = 1;
    let tmp = match b.pop() {
        Some(n) => {
            for _ in 0..n {
                ret *= a;
                ret %= 1337;
            }
            super_pow(a, b)
        },
        None => 1,
    };
    for _ in 0..10 {
        ret *= tmp;
        ret %= 1337;
    }
    ret
}

pub fn super_pow_1(a: i32, b: Vec<i32>) -> i32 {
    const C: i32 = 1337;
    const PHI: i32 = 1140;  // PHI(1337) = 1140 according to euler
    let n = b.len();
    if n == 0 {
        return 1;
    }
    let mut exp = 0;
    for digit in b {
        exp = (10 * exp + digit) % PHI;
    }
    quick_pow(a as i64, exp as i64, C as i64) as i32
}

fn quick_pow(mut a: i64, mut b: i64, c: i64) -> i64 {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res*a)%c;
        }
        b >>=1;
        a = (a*a)%c;
    }
    res
}