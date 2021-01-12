use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use data_structure::union_find::UnionFind;

pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    if pairs.len() == 0 {
        return s;
    }
    let len = s.len();
    let mut union_find = UnionFind::new(len);
    for pair in pairs {
        let index1 = pair[0];
        let index2 = pair[1];
        union_find.union(index1 as usize, index2 as usize);
    }
    let s = s.into_bytes();
    let mut map = HashMap::<usize, BinaryHeap<Reverse<u8>>>::new();
    for i in 0..len {
        let root = union_find.find(i);
        if map.contains_key(&root) {
            map.get_mut(&root).unwrap().push(Reverse(s[i]));
        }
        else {
            let mut min_heap = BinaryHeap::<Reverse<u8>>::new();
            min_heap.push(Reverse(s[i]));
            map.insert(root, min_heap);
        }
    }

    let mut ret = Vec::<u8>::with_capacity(len);

    for i in 0..len {
        let root = union_find.find(i);
        let c = map.get_mut(&root).unwrap().pop().unwrap().0;
        ret.push(c);
    }
    String::from_utf8(ret).unwrap()
}