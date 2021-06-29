

pub fn convert_to_title(column_number: i32) -> String {
    let mut s = String::with_capacity(8);
    let mut n = column_number;
    while n != 0 {
        let c = (((n - 1) % 26) as u8 + b'A') as char;
        s.insert(0, c);
        n = (n - 1) / 26;
    }
    s
}