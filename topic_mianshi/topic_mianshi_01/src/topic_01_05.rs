pub fn one_edit_away(first: String, second: String) -> bool {
    let first = first.into_bytes();
    let second = second.into_bytes();
    let len1 = first.len();
    let len2 = second.len();
    if len1 > len2 {
        if len1 - len2 > 1 {
            return false;
        }
        let (mut idx1, mut idx2) = (0, 0);
        while idx1 < len1 && idx2 < len2 {
            if first[idx1] == second[idx2] {
                idx1 += 1;
                idx2 += 1;
            } else {
                if idx1 == idx2 {
                    idx1 += 1;
                } else {
                    return false;
                }
            }
        }
    } else if len1 == len2 {
        return first
            .iter()
            .zip(second.iter())
            .filter(|(c1, c2)| c1 != c2)
            .count()
            <= 1;
    } else {
        if len2 - len1 > 1 {
            return false;
        }
        let (mut idx1, mut idx2) = (0, 0);
        while idx1 < len1 && idx2 < len2 {
            if first[idx1] == second[idx2] {
                idx1 += 1;
                idx2 += 1;
            } else {
                if idx1 == idx2 {
                    idx2 += 1;
                } else {
                    return false;
                }
            }
        }
    }
    true
}
