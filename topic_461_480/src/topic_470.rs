

pub fn rand10() -> i32 {
    let (mut a, mut b) = (rand7(), rand7() * 7);
    while a + b > 47 {
        a = rand7();
        b = rand7() * 7;
    }
    (a + b + 2) % 10 + 1
}

fn rand7() -> i32 {
    0
}