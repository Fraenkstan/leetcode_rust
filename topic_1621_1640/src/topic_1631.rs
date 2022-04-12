use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct Item(usize, usize, i32);

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.2.partial_cmp(&self.2)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        // self.cnt.cmp(&other.cnt)
        other.2.cmp(&self.2)
    }
}

//timeout
#[allow(dead_code)]
pub fn minimum_effort_path_timeout(heights: Vec<Vec<i32>>) -> i32 {
    let rows = heights.len();
    if rows == 0 {
        return 0;
    }
    let cols = heights[0].len();
    if cols == 0 {
        return 0;
    }

    //记录到各个点的体力
    let mut efforts = vec![vec![std::i32::MAX; cols]; rows];

    let mut stack = Vec::new();
    stack.push((0, 0));
    efforts[0][0] = 0;

    let dir: Vec<i32> = vec![0, 1, 0, -1, 0];
    while let Some((x, y)) = stack.pop() {
        //当前位置

        //对每一方向
        dir.windows(2).for_each(|dir| {
            let dx = dir[0];
            let dy = dir[1];

            if let Some(row) = efforts.get(x + dx as usize) {
                if let Some(v) = row.get(y + dy as usize) {
                    //当前体力 和 从当前到(x+dy,y+dy)高度差的绝对值
                    let tmph = heights[x][y] - heights[x + dx as usize][y + dy as usize];
                    let new_effort = std::cmp::max(efforts[x][y], tmph.abs());

                    if new_effort < *v {
                        // efforts[x + dx as usize][y + dy as usize] = new_effort;
                        *efforts
                            .get_mut(x + dx as usize)
                            .unwrap()
                            .get_mut(y + dy as usize)
                            .unwrap() = new_effort;
                        stack.push((x + dx as usize, y + dy as usize));
                    }
                }
            }
        });
    }

    efforts[rows - 1][cols - 1]
}

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let rows = heights.len();
    if rows == 0 {
        return 0;
    }
    let cols = heights[0].len();
    if cols == 0 {
        return 0;
    }

    //记录到各个点的体力
    let mut efforts = vec![vec![std::i32::MAX; cols]; rows];
    let mut seen = vec![vec![0; cols]; rows];

    let mut heap = BinaryHeap::new();
    let dir: Vec<i32> = vec![0, 1, 0, -1, 0];

    heap.push(Item(0, 0, 0));
    efforts[0][0] = 0;

    while let Some(cur) = heap.pop() {
        if cur.0 == rows - 1 && cur.1 == cols - 1 {
            return cur.2;
        }
        //已经处理过了. 因为相同点可能会加入多次
        if 1 == seen[cur.0][cur.1] {
            continue;
        }
        seen[cur.0][cur.1] = 1;
        //对每一方向
        dir.windows(2).for_each(|dir| {
            let dx = dir[0];
            let dy = dir[1];

            if cur.0 == 0 && dx == -1 {
                return;
            }

            if let Some(row) = efforts.get((cur.0 as i32 + dx) as usize) {
                if cur.1 == 0 && dy == -1 {
                    return;
                }
                if let Some(v) = row.get((cur.1 as i32 + dy) as usize) {
                    //当前体力 和 从当前到(x+dy,y+dy)高度差的绝对值
                    let tmph = heights[cur.0][cur.1]
                        - heights[(cur.0 as i32 + dx) as usize][(cur.1 as i32 + dy) as usize];
                    let new_effort = std::cmp::max(cur.2, tmph.abs());

                    if new_effort < *v {
                        // efforts[x + dx as usize][y + dy as usize] = new_effort;
                        *efforts
                            .get_mut((cur.0 as i32 + dx) as usize)
                            .unwrap()
                            .get_mut((cur.1 as i32 + dy) as usize)
                            .unwrap() = new_effort;

                        heap.push(Item(
                            (cur.0 as i32 + dx) as usize,
                            (cur.1 as i32 + dy) as usize,
                            new_effort,
                        ));
                    }
                }
            }
        });
    }
    0
}
