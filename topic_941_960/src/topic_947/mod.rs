use std::collections::{HashSet};
use data_structure::union_find::dsu::DSU;

pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {

    let mut dsu = DSU::new();
    let mut set = HashSet::new();
    for stone in stones.iter() {
        dsu.union(stone[0] * 2, stone[1] * 2 + 1);
    }
    for stone in stones.iter() {
        set.insert(dsu.find(stone[0] * 2));
    }
    (stones.len() - set.len()) as i32

}