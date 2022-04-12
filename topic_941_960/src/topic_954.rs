

pub fn can_reorder_doubled(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let mut tmp = vec![];
    for num in arr {
        if tmp.len() == 0 {
            tmp.push(num);
        } else {
            let cmp = *tmp.first().unwrap();
            if cmp > 0 {
                if cmp * 2 < num {
                    return false;
                } else if cmp * 2 == num {
                    tmp.remove(0);
                } else {
                    tmp.push(num);
                }
            } else {
                if num * 2 == cmp {
                    tmp.remove(0);
                } else if num * 2 > cmp {
                    return false;
                } else {
                    tmp.push(num);
                }
            }
        }
    }
    tmp.len() == 0
}