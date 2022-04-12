pub fn search(arr: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, arr.len() - 1);
    while arr[left] == arr[right] {
        right -= 1;
    }
    if arr[left] > arr[right] {
        while right > left {
            let mid = left + (right - left) / 2;
            if arr[mid] < arr[0] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
    }
    if left == right {
        if target >= arr[0] {
            left = 0;
        } else {
            right = arr.len() - 1;
        }
    }
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            right = mid;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return if arr[right] == target {
        right as i32
    } else {
        -1
    };
}
