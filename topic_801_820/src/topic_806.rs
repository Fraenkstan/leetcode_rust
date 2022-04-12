

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut line = 0;
    let mut words = 0;
    for c in s.chars() {
        let index = c as usize - 'a' as usize;
        if words + widths[index] <= 100 {
            words += widths[index];
        } else {
            line += 1;
            words = widths[index];
        }
    }
    let mut ans = Vec::new();
    ans.push(line + 1);
    ans.push(words);
    ans
}