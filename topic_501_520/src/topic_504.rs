pub fn convert_to_base7(mut num: i32) -> String {
    if num == 0 {
        return "0".to_string();
    }
    let mut builder = String::new();
    let mut flag = false;
    if num < 0 {
        num = -1 * num;
        flag = true;
    }
    while num > 0 {
        let i = num % 7;
        builder.push((i as u8 + '0' as u8) as char);
        num = num / 7;
    }
    if flag {
        builder.push('-');
    }
    builder.chars().rev().collect::<String>()
}
