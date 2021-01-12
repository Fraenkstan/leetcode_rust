#[derive(Debug)]
pub struct UnionFind {
    parent : Vec<usize>,
    rank : Vec<usize>
}

impl UnionFind {

    pub fn new(n: usize) -> UnionFind {
        UnionFind{
            parent : {
                let mut  vec: Vec<usize> = Vec::new();
                for i in 0..n {
                    vec.push(i);
                }
                vec
            },
            rank : {
                let mut  vec: Vec<usize> = Vec::new();
                for _i in 0..n {
                    vec.push(1);
                }
                vec
            },
        }
    }

    pub fn find(&self, x: usize) -> usize {
        if x == self.parent[x] {
            return x;
        }
        else {
            self.find(self.parent[x])
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {return;}

        if self.rank[root_x] == self.rank[root_y] {
            if let Some(parent) = self.parent.get_mut(root_x) {
                *parent = root_y;
            }
            if let Some(rank) = self.rank.get_mut(root_y) {
                *rank += 1;
            }
        }
        else if self.rank[root_x] < self.rank[root_y] {
            if let Some(parent) = self.parent.get_mut(root_x) {
                *parent = root_y;
            }
        }
        else {
            if let Some(parent) = self.parent.get_mut(root_y) {
                *parent = root_x;
            }
        }
    }
}