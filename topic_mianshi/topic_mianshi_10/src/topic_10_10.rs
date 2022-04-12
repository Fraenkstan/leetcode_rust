use std::collections::BTreeMap;

struct StreamRank {
    map: BTreeMap<i32, i32>,
}

#[allow(unused)]
impl StreamRank {
    fn new() -> Self {
        StreamRank {
            map: BTreeMap::new(),
        }
    }

    fn track(&mut self, x: i32) {
        *self.map.entry(x).or_insert(0) += 1;
    }

    fn get_rank_of_number(&self, x: i32) -> i32 {
        self.map
            .iter()
            .filter(|(&k, _v)| k <= x)
            .map(|(_k, v)| v)
            .sum()
    }
}
