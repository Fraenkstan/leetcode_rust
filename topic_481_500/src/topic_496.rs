use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums2.len() {
        let mut j = i;
        while j < nums2.len() && nums2[j] <= nums2[i] {
            j += 1;
        }
        if j == nums2.len() {
            map.insert(nums2[i], -1);
        } else {
            map.insert(nums2[i], nums2[j]);
        }
    }
    nums1
        .iter()
        .map(|num| *map.get(num).unwrap_or(&-1))
        .collect()
}

pub fn next_greater_element1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut stack = vec![];
    for i in (0..nums2.len()).rev() {
        while !stack.is_empty() && nums2[i] >= *stack.last().unwrap() {
            stack.pop();
        }
        map.insert(
            nums2[i],
            if stack.is_empty() {
                -1
            } else {
                *stack.last().unwrap()
            },
        );
        stack.push(nums2[i]);
    }
    nums1
        .iter()
        .map(|num| *map.get(num).unwrap_or(&-1))
        .collect()
}

#[test]
fn topic_496() {
    println!(
        "{:?}",
        next_greater_element1(vec![4, 1, 2], vec![1, 3, 4, 2])
    );
    println!("{:?}", next_greater_element1(vec![2, 4], vec![1, 2, 3, 4]));
}
