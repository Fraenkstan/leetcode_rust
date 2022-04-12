use trace::trace;

trace::init_depth_var!();

#[trace(prefix_enter = "ENTER: ", prefix_exit = "EXIT: ")]
pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter()
        .map(|x| x.iter().rev().map(|x| x ^ 1).collect())
        .collect()
}
