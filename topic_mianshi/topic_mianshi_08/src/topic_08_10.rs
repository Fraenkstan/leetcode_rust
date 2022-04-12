pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
    let sr = sr as usize;
    let sc = sc as usize;
    if image[sr][sc] != new_color {
        dfs(sr, sc, image[sr][sc], new_color, &mut image);
    }
    image
}

fn dfs(x: usize, y: usize, origin: i32, target: i32, image: &mut Vec<Vec<i32>>) {
    image[x][y] = target;
    let length = image[0].len();
    let width = image.len();
    if x < width - 1 && origin == image[x + 1][y] {
        dfs(x + 1, y, origin, target, image);
    }
    if x > 0 && origin == image[x - 1][y] {
        dfs(x - 1, y, origin, target, image);
    }
    if y < length - 1 && origin == image[x][y + 1] {
        dfs(x, y + 1, origin, target, image);
    }
    if y > 0 && origin == image[x][y - 1] {
        dfs(x, y - 1, origin, target, image);
    }
}
