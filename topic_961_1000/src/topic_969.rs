pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let arr = &mut arr;
    let n = arr.len();
    let mut ans = vec![];
    for i in (1..=n).rev() {
        let (idx, _) = arr[0..i]
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| **v)
            .unwrap();
        if idx + 1 != i {
            arr[..=idx].reverse();
            arr[..i].reverse();
            ans.push((idx + 1) as i32);
            ans.push(i as i32);
        }
    }
    ans
}
