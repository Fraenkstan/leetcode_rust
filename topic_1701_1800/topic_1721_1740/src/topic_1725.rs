
pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
    let squares = rectangles
        .iter()
        .map(|rect| rect.iter().min().unwrap())
        .collect::<Vec<&i32>>();
    let max = squares.iter().max().unwrap();
    squares.iter().filter(|x| *x == max).count() as i32
}