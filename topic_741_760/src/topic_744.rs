

pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let (mut left, mut right) = (0usize, letters.len() - 1);
    while right > left {
        let mid = (left + right) / 2;
        if target >= letters[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return if letters[left] > target { letters[left] } else { letters[0] }
}