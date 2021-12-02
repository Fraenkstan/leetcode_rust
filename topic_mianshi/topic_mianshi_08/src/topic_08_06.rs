

pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
    move_plate(a.len(), a, b, c);
}

fn move_plate(num: usize, a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
    if num == 1 {
        let n = a.pop().unwrap();
        c.push(n);
        return;
    }
    move_plate(num - 1, a, c, b);
    let n = a.pop().unwrap();
    c.push(n);
    move_plate(num - 1, b, a, c);
}