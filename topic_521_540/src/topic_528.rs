use rand::{thread_rng, Rng};

pub struct Solution {
    weight: Vec<i32>
}

impl Solution {

    pub(crate) fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i - 1];
        }
        Solution{
            weight: w
        }
    }

    pub(crate) fn pick_index(&self) -> i32 {
        let x = thread_rng().gen_range(1, self.weight.last().unwrap() + 1);
        match self.weight.binary_search(&x) {
            Ok(i) => i as i32,
            Err(i) => i as i32
        }
    }
}