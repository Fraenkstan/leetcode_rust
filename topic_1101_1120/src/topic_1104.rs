pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
    let mut max = 2;
    while max <= label {
        max <<= 1;
    }
    let mut result = Vec::new();
    let mut now = label;
    while now > 0 {
        result.push(now);
        now = (max >> 2) + (max >> 1) - 1 - (now >> 1);
        max >>= 1;
    }
    result.reverse();
    return result;
}
