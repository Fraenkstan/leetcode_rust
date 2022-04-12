use std::collections::VecDeque;

pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut g = grid;
    let (n, m) = (g.len(), g[0].len());
    let mut fifo = VecDeque::with_capacity(n * m);
    for i in 0..n {
        for j in 0..m {
            if g[i][j] == 0 {
                continue;
            }
            g[i][j] = 0;
            fifo.push_back((i as i32, j as i32));
            ans += bfs(&mut g, &mut fifo);
        }
    }
    ans
}

fn bfs(g: &mut Vec<Vec<i32>>, fifo: &mut VecDeque<(i32, i32)>) -> i32 {
    let dir = [-1, 0, 1, 0, -1];
    let (n, m) = (g.len() as i32, g[0].len() as i32);
    let mut boundary = false;
    let mut cnt = 0;
    while let Some((i, j)) = fifo.pop_front() {
        cnt += 1;
        for d in 0..4 {
            let x = i + dir[d];
            let y = j + dir[d + 1];
            if x < 0 || x >= n || y < 0 || y >= m {
                boundary = true;
                continue;
            }
            if g[x as usize][y as usize] == 1 {
                g[x as usize][y as usize] = 0;
                fifo.push_back((x, y));
            }
        }
    }
    if boundary {
        0
    } else {
        cnt
    }
}
