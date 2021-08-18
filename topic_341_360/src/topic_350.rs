use std::cmp::Ordering;

pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    nums1.sort();
    nums2.sort();
    let mut ans = vec![];
    let (mut i, mut j) = (0usize, 0usize);
    while i < nums1.len() && j < nums2.len() {
        match nums1[i].cmp(&nums2[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                ans.push(nums1[i]);
                i += 1;
                j += 1;
            }
        }
    }
    ans
}