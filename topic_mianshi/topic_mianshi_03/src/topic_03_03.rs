pub struct StackOfPlates {
    vec: Vec<i32>,
    capacity: i32,
    index: Vec<Vec<usize>>,
}

impl StackOfPlates {
    pub(crate) fn new(cap: i32) -> Self {
        StackOfPlates {
            vec: vec![],
            capacity: cap,
            index: vec![vec![]],
        }
    }

    pub(crate) fn push(&mut self, val: i32) {
        if self.capacity == 0 {
            return;
        }
        self.vec.push(val);
        for index in self.index.iter_mut() {
            if index.len() < self.capacity as usize {
                index.push(self.vec.len() - 1);
                return;
            }
        }
        self.index.push(vec![self.vec.len() - 1]);
    }

    pub(crate) fn pop(&mut self) -> i32 {
        if self.vec.len() == 0 {
            return -1;
        }
        let index = self.vec.len() - 1;
        for (i, index_vec) in self.index.iter_mut().enumerate() {
            if index_vec.contains(&index) {
                index_vec.pop();
                if index_vec.len() == 0 {
                    self.index.remove(i);
                }
                break;
            }
        }
        self.vec.pop().unwrap_or(-1)
    }

    pub(crate) fn pop_at(&mut self, index: i32) -> i32 {
        if self.index.len() <= index as usize || self.capacity == 0 {
            return -1;
        }
        let index_vec = self.index.get_mut(index as usize).unwrap();
        let mut index = index_vec.pop().unwrap();
        if index_vec.len() == 0 {
            self.index.remove(index as usize);
        }
        for index_vec in self.index.iter_mut() {
            index_vec
                .into_iter()
                .filter(|i| *i > &mut index)
                .for_each(|i| *i -= 1);
        }
        self.vec.remove(index)
    }
}
