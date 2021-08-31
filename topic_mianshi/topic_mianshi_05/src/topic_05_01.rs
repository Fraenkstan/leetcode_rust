

pub fn insert_bits(mut n: i32, mut m: i32, i: i32, j: i32) -> i32 {
    println!("m: {:b}, n: {:b}", m, n);
    let mut mask=((1 << (j - i + 1)) - 1) << i;
    println!("mask: {:b}", mask);
    mask = !mask;
    println!("!mask: {:b}", mask);
    n &= mask;
    println!("n: {:b}", n);
    m = m << i;
    println!("m: {:b}", m);
    println!("m | n : {:b}", m | n);
    m | n
}