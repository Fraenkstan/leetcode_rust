struct Heap(
    Vec<Vec<i32>>,
    std::collections::BinaryHeap<(i32, [u8; 2])>,
    u8,
);

impl Heap {
    fn new(mut grid: Vec<Vec<i32>>, l: u8) -> Self {
        let mut h = std::collections::BinaryHeap::new();
        h.push((-grid[0][0], [0, 0]));
        grid[0][0] = -1;
        Self(grid, h, l)
    }

    fn push(&mut self, v: [u8; 2]) {
        if v[0] < self.2 && v[1] < self.2 && self.0[v[0] as usize][v[1] as usize] >= 0 {
            self.1.push((-self.0[v[0] as usize][v[1] as usize], v));
            self.0[v[0] as usize][v[1] as usize] = -1;
        }
    }
}

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let l = grid.len() as u8 - 1;
    let mut t = Heap::new(grid, l + 1);
    let mut ans = 0;
    while let Some((deep, [v0, v1])) = t.1.pop() {
        ans = ans.min(deep);
        if v0 == v1 && v0 == l {
            break;
        }
        match (v0, v1) {
            (0, 0) => (&[[v0, v1 + 1], [v0 + 1, v1]])
                .iter()
                .for_each(|&x| t.push(x)),
            (0, _) => (&[[v0, v1 + 1], [v0 + 1, v1], [v0, v1 - 1]])
                .iter()
                .for_each(|&x| t.push(x)),
            (_, 0) => (&[[v0, v1 + 1], [v0 + 1, v1], [v0 - 1, v1]])
                .iter()
                .for_each(|&x| t.push(x)),
            (_, _) => (&[[v0, v1 + 1], [v0, v1 - 1], [v0 + 1, v1], [v0 - 1, v1]])
                .iter()
                .for_each(|&x| t.push(x)),
        }
    }
    -ans
}
