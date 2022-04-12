pub struct SortedStack {
    stack: Vec<i32>,
}

impl SortedStack {
    pub(crate) fn new() -> Self {
        SortedStack { stack: vec![] }
    }

    pub(crate) fn push(&mut self, val: i32) {
        let mut tmp = vec![];
        while !self.is_empty() && self.peek() <= val {
            tmp.push(self.stack.pop().unwrap());
        }
        tmp.push(val);
        while let Some(val) = tmp.pop() {
            self.stack.push(val);
        }
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn peek(&self) -> i32 {
        *self.stack.last().unwrap_or(&-1)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
