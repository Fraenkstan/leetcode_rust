
pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
    let mut arr = arr;
    arr.sort_unstable();
    let n = arr.len();
    if arr[0] != 1 { arr[0] = 1; }
    for i in 1..n {
        if arr[i] - arr[i - 1] > 1 {
            arr[i] = arr[i - 1] + 1;
        }
    }
    *arr.iter().max().unwrap()
}