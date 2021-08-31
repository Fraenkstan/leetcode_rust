


pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    (1..=arr.len()).step_by(2).into_iter().map(|len| {
        arr.windows(len).into_iter().map(|window|
            window.iter().sum::<i32>()
        ).sum::<i32>()
    }).sum()
}