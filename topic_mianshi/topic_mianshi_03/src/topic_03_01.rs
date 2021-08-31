pub struct TripleInOne {
    vec: Vec<i32>,
    index: [usize; 3]
}

impl TripleInOne {

    pub(crate) fn new(stack_size: i32) -> Self {
        TripleInOne{vec: vec![0; 3 * stack_size as usize], index: [0,1,2]}
    }

    pub(crate) fn push(&mut self, stack_num: i32, value: i32) {
        if self.index[stack_num as usize] < self.vec.len() {
            self.vec[self.index[stack_num as usize]] = value;
            self.index[stack_num as usize] += 3;
        }
    }

    pub(crate) fn pop(&mut self, stack_num: i32) -> i32 {
        match stack_num as usize {
            n if self.index[n] >= 3 => {
                self.index[n] -= 3;
                self.vec[self.index[n]]
            }
            _ => -1
        }
    }

    pub(crate) fn peek(&self, stack_num: i32) -> i32 {
        return if self.index[stack_num as usize] >= 3 {
            self.vec[self.index[stack_num as usize] - 3]
        } else { -1 }
    }

    pub(crate) fn is_empty(&self, stack_num: i32) -> bool {
        self.index[stack_num as usize] < 3
    }
}