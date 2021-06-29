use std::collections::HashMap;

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len() as i32;
    if n < 3 {
        return n;
    }
    let mut ans = 0;
    (0..n as usize).into_iter().for_each(|i| {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;
        (i + 1..n as usize).into_iter().for_each(|j| {
            let mut dx = points[j][0] - points[i][0];
            let mut dy = points[j][1] - points[i][1];
            if dx == 0 {
                dy = 1;
            } else if dy == 0 {
                dx = 1;
            } else {
                if dy < 0 {
                    dx = -dx;
                    dy = -dy;
                }
                let gcd_xy = gcd(dx, dy);
                dx /= gcd_xy;
                dy /= gcd_xy;
            }
            let cnt = *map.entry(dy + 20001 * dx).or_insert(0) + 1;
            count = count.max(cnt);
            map.insert(dy + 20001 * dx, cnt);
        });
        ans = ans.max(count);
    });
    ans + 1
}

fn gcd(a: i32, b: i32) -> i32 {
    if b != 0 { gcd(b, a % b) } else { a }
}

#[allow(unused)]
pub fn max_points_1(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len() as i32;
    if n < 3 {
        return n;
    }
    let mut ans = 0;
    (0..n as usize).into_iter().for_each(|i| {
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut count = 0;
        (i + 1..n as usize).into_iter().for_each(|j| {
            let dx = points[j][0] - points[i][0];
            let dy = points[j][1] - points[i][1];
            if dx == 0 { count += 1; } else {
                let d: String = (dy as f64 / dx as f64).to_string();
                let cnt = map.get(d.as_str()).or_else(|| Some(&0)).unwrap() + 1;
                map.insert(d, cnt);
                count = count.max(cnt);
            }
        });
        ans = ans.max(count);
    });
    ans + 1
}