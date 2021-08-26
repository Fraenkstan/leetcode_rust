

pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    ghosts.iter().all(|ghost| {
        (ghost[0] - target[0]).abs() + (ghost[1] - target[1]).abs() > target[0].abs() + target[1].abs()
    })
}