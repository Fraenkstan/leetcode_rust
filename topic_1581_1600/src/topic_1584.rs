use std::collections::HashSet;

pub fn kruskal(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut edges = Vec::new();
    for i in 0..n - 1 {
        for j in i + 1..n {
            edges.push((
                i,
                j,
                (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs(),
            ));
        }
    }
    edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));

    let mut parents = Vec::new();
    for i in 0..n {
        parents.push(i);
    }

    let mut answer = 0;
    for edge in edges {
        let mut a = edge.0;
        let mut b = edge.1;
        while a != parents[a] {
            a = parents[a];
        }
        while b != parents[b] {
            b = parents[b];
        }
        if a == b {
            continue;
        }
        parents[b] = a;
        answer += edge.2;
    }
    answer
}

pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut edges = Vec::new();
    solve(points, n, &mut edges);

    let mut dsu = DisjointSetUnion::new(n);
    edges.sort_by(|a, b| a.len.cmp(&b.len));
    let (mut ret, mut num) = (0, 1);
    for edge in edges.iter() {
        let (len, x, y) = (edge.len, edge.x, edge.y);
        if dsu.union_set(x, y) {
            ret += len;
            num += 1;
            if num == n {
                break;
            }
        }
    }
    ret
}

fn solve(points: Vec<Vec<i32>>, n: usize, edges: &mut Vec<Edge>) {
    let mut pos: Vec<Pos> = Vec::with_capacity(n);
    for i in 0..n {
        pos.push(Pos::new(i, points[i][0], points[i][1]));
    }
    build(n, &mut pos, edges);
    for i in 0..n {
        let temp = pos[i].x;
        pos[i].x = pos[i].y;
        pos[i].y = temp;
    }
    build(n, &mut pos, edges);
    for i in 0..n {
        pos[i].x = -pos[i].x;
    }
    build(n, &mut pos, edges);
    for i in 0..n {
        let temp = pos[i].x;
        pos[i].x = pos[i].y;
        pos[i].y = temp;
    }
    build(n, &mut pos, edges);
}

fn build(n: usize, pos: &mut Vec<Pos>, edges: &mut Vec<Edge>) {
    pos.sort_by(|a, b| {
        return if a.x == b.x {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        };
    });
    let mut a = Vec::with_capacity(n);
    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..n as usize {
        a.push(pos[i].y - pos[i].x);
        set.insert(pos[i].y - pos[i].x);
    }
    let num = set.len();
    let mut b: Vec<i32> = Vec::with_capacity(num);
    for element in set.iter() {
        b.push(*element);
    }
    b.sort();
    let mut bit = BIT::new((num + 1) as i32);
    let mut i = n - 1;
    while i > 0 as usize {
        let poss = binary_search(&b, a[i]) + 1;
        let j = bit.query(poss);
        if j != -1 {
            let j = j as usize;
            let dis = (pos[i].x - pos[j].x).abs() + (pos[i].y - pos[j].y).abs();
            edges.push(Edge::new(dis, pos[i].id, pos[j].id));
        }
        bit.update(poss, pos[i].x + pos[i].y, i as i32);
        i -= 1;
    }
}

fn binary_search(array: &Vec<i32>, target: i32) -> i32 {
    let mut low = 0;
    let mut high = array.len() as i32 - 1;
    while low < high {
        let mid = (high - low) / 2 + low;
        let num = array[mid as usize];
        if num < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[allow(dead_code)]
struct DisjointSetUnion {
    f: Vec<usize>,
    rank: Vec<i32>,
    n: usize,
}

struct BIT {
    tree: Vec<i32>,
    id_rec: Vec<i32>,
    n: i32,
}

struct Edge {
    len: i32,
    x: usize,
    y: usize,
}

struct Pos {
    id: usize,
    x: i32,
    y: i32,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> DisjointSetUnion {
        DisjointSetUnion {
            f: {
                let mut vec = Vec::new();
                for _i in 0..n {
                    vec.push(1);
                }
                vec
            },
            rank: {
                let mut vec = Vec::new();
                for i in 0..n {
                    vec.push(i as i32);
                }
                vec
            },
            n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x == self.f[x] {
            return x;
        }
        let val = self.find(self.f[x]);
        self.f[x] = val;
        val
    }

    pub fn union_set(&mut self, x: usize, y: usize) -> bool {
        let (mut fx, mut fy) = (self.find(x), self.find(y));
        if fx == fy {
            return false;
        }
        if self.rank[fx] < self.rank[fy] {
            let temp = fx;
            fx = fy;
            fy = temp;
        }
        self.rank[fx] += self.rank[fy];
        self.f[fy] = fx;
        true
    }
}

impl BIT {
    pub fn new(n: i32) -> BIT {
        BIT {
            tree: {
                let mut vec = Vec::new();
                for _i in 0..n {
                    vec.push(std::i32::MAX);
                }
                vec
            },
            id_rec: {
                let mut vec = Vec::new();
                for _i in 0..n {
                    vec.push(-1);
                }
                vec
            },
            n,
        }
    }

    pub fn low_bit(&self, k: i32) -> i32 {
        k & (-k)
    }

    pub fn update(&mut self, mut pos: i32, val: i32, id: i32) {
        while pos > 0 {
            if self.tree[pos as usize] > val {
                self.tree[pos as usize] = val;
                self.id_rec[pos as usize] = id;
            }
            pos -= self.low_bit(pos);
        }
    }

    pub fn query(&self, mut pos: i32) -> i32 {
        let mut min = i32::MAX;
        let mut j = -1;
        while pos < self.n {
            if min > self.tree[pos as usize] {
                min = self.tree[pos as usize];
                j = self.id_rec[pos as usize];
            }
            pos += self.low_bit(pos);
        }
        j
    }
}

impl Edge {
    pub fn new(len: i32, x: usize, y: usize) -> Edge {
        Edge { len, x, y }
    }
}

impl Pos {
    pub fn new(id: usize, x: i32, y: i32) -> Pos {
        Pos { id, x, y }
    }
}
