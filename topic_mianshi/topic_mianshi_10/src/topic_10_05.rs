pub fn find_string(words: Vec<String>, s: String) -> i32 {
    let (mut left, mut right) = (0, words.len() - 1);
    while words[left] == "".to_string() {
        left += 1;
    }
    while words[right] == "".to_string() {
        right -= 1;
    }
    while left < right {
        let mut mid = left + (right - left) / 2;
        let tmp = mid;
        while words[mid] == "".to_string() && mid < right {
            mid += 1;
        }
        if mid == right {
            right = tmp;
        } else if words[mid] == s {
            return mid as i32;
        } else if words[mid] > s {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return if words[right] == s { right as i32 } else { -1 };
}
