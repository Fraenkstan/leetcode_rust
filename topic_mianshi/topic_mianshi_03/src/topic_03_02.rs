pub struct MinStack {
    vec: Vec<i32>,
    min: Vec<usize>,
}

impl MinStack {
    pub(crate) fn new() -> Self {
        MinStack {
            vec: vec![],
            min: vec![],
        }
    }

    pub(crate) fn push(&mut self, x: i32) {
        self.vec.push(x);
        if x <= self.get_min() {
            self.min.push(self.vec.len() - 1);
        }
    }

    pub(crate) fn pop(&mut self) {
        let min = self.get_min();
        if let Some(x) = self.vec.pop() {
            if x == min {
                self.min.pop();
            }
        }
    }

    pub(crate) fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    pub(crate) fn get_min(&self) -> i32 {
        if self.min.len() == 0 {
            return i32::MAX;
        }
        self.vec[self.min[self.min.len() - 1]]
    }
}
