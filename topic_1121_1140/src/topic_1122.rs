

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut map = vec![0; 1001];
    let mut result = Vec::with_capacity(arr1.len());
    for a1 in arr1 {
        map[a1 as usize] += 1;
    }
    for a2 in arr2 {
        let ua2 = a2 as usize;
        while map[ua2] > 0 {
            result.push(a2);
            map[ua2] -= 1;
        }
    }
    for (i, mut m) in map.into_iter().enumerate() {
        while m > 0 {
            result.push(i as i32);
            m -= 1;
        }
    }
    result
}