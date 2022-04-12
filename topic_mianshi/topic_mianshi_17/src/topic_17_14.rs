pub fn smallest_k(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
    let len = arr.len();
    if len == 0 {
        return vec![];
    }
    quick_sort(&mut arr, 0, len - 1, k as usize);
    arr[0..k as usize].to_vec()
}

fn quick_sort(arr: &mut Vec<i32>, start: usize, end: usize, k: usize) {
    if start >= end {
        return;
    }
    let end_val = arr[end];
    let (mut p0, mut p1) = (start, start);
    while p1 < end {
        if arr[p1] < end_val {
            arr.swap(p0, p1);
            p0 += 1;
        }
        p1 += 1;
    }
    arr.swap(p0, end);
    if p0 < k {
        quick_sort(arr, p0 + 1, end, k);
    } else {
        quick_sort(arr, start, p0 - 1, k);
    }
}
