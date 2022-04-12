pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    for i in left..=right {
        if !covered(ranges.as_ref(), i) {
            return false;
        }
    }
    true
}

fn covered(ranges: &Vec<Vec<i32>>, num: i32) -> bool {
    ranges.into_iter().any(|vec| vec[0] <= num && num <= vec[1])
}
