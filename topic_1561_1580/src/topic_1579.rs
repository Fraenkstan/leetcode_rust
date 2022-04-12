use std::cmp::{max, min};

pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut f: Vec<i32> = Vec::with_capacity((n + 1) as usize);
    let mut ans = 0;

    f.push(0);
    for i in 1..n + 1 {
        f.push(i);
        // f[i as usize] = i;
    }
    let mut a: Vec<Vec<i32>> = Vec::new();
    let mut b: Vec<Vec<i32>> = Vec::new();
    let mut c: Vec<Vec<i32>> = Vec::new();
    for i in 0..edges.len() {
        match edges[i][0] {
            1 => a.push(edges[i].clone()),
            2 => b.push(edges[i].clone()),
            _ => c.push(edges[i].clone()),
        }
    }

    for vec in c.iter() {
        if !merge(&mut f, vec[1], vec[2]) {
            ans += 1
        }
    }
    let mut ff = f.clone();

    for vec in a.iter() {
        if !merge(&mut f, vec[1], vec[2]) {
            ans += 1
        }
    }
    for vec in b.iter() {
        if !merge(&mut ff, vec[1], vec[2]) {
            ans += 1
        }
    }

    let fa = find_f(&mut f, 1);
    let fb = find_f(&mut ff, 1);

    for i in 1..n {
        if fa != find_f(&mut f, i) || fb != find_f(&mut ff, i) {
            return -1;
        }
    }
    ans
}

fn find_f(f: &mut Vec<i32>, mut x: i32) -> i32 {
    let mut root = x;
    while root != f[root as usize] {
        root = f[root as usize];
    }
    while f[x as usize] != root {
        let z = f[x as usize];
        f[x as usize] = root;
        x = z;
    }
    root
}

fn merge(f: &mut Vec<i32>, a: i32, b: i32) -> bool {
    let fa = find_f(f, a);
    let fb = find_f(f, b);
    if fa != fb {
        f[max(fa, fb) as usize] = min(fa, fb);
        return true;
    }
    false
}
