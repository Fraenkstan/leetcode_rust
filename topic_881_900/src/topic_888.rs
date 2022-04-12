use std::collections::HashSet;

pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let d = ((a.iter().sum::<i32>() as i32) - (b.iter().sum::<i32>() as i32)) >> 1;

    let mut set = HashSet::new();
    b.iter().for_each(|u| {
        set.insert(*u);
    });
    for t in a {
        if set.contains(&(t - d)) {
            return vec![t, t - d];
        }
    }
    return vec![];
}

#[allow(dead_code)]
pub fn fair_candy_swap_1(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
    [a.sort_unstable(), b.sort_unstable()];
    let t = &((a.iter().sum::<i32>() - b.iter().sum::<i32>()) >> 1);
    let [mut aa, mut bb] = [a.into_iter(), b.into_iter()];
    let [mut i, mut j] = [aa.next().unwrap(), bb.next().unwrap()];
    return loop {
        match (i - j).cmp(t) {
            std::cmp::Ordering::Less => i = aa.next().unwrap(),
            std::cmp::Ordering::Equal => break vec![i, j],
            std::cmp::Ordering::Greater => j = bb.next().unwrap(),
        }
    };
}
