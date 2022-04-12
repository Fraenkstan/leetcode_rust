use data_structure::union_find::UnionFind;
use std::collections::HashSet;

pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut dsu = UnionFind::new(stones.len());
    let mut set = HashSet::new();
    for stone in stones.iter() {
        dsu.union((stone[0] * 2) as usize, (stone[1] * 2 + 1) as usize);
    }
    for stone in stones.iter() {
        set.insert(dsu.find((stone[0] * 2) as usize));
    }
    (stones.len() - set.len()) as i32
}
