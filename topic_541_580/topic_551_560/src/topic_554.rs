use std::collections::HashMap;

pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
    wall.len() as i32 - wall.iter().fold(HashMap::new(), |mut map, width| {
        width.iter().rev().skip(1).rev().fold(0, |mut sum, &i| {
            sum += i;
            *map.entry(sum).or_insert(0) += 1;
            sum
        });
        map
    }).iter().map(|(_k, &v)| v).max().unwrap_or(0)
}