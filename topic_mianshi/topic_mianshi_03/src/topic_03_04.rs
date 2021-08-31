use std::option::Option::Some;

pub struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>
}

impl MyQueue {

    pub(crate) fn new() -> Self {
        MyQueue{
            stack_in: vec![],
            stack_out: vec![]
        }
    }

    pub(crate) fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    pub(crate) fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while let Some(val) = self.stack_in.pop() {
                self.stack_out.push(val);
            }
        }
        self.stack_out.pop().unwrap_or(-1)
    }

    pub(crate) fn peek(&self) -> i32 {
        return if !self.stack_in.is_empty() {
            *self.stack_in.first().unwrap_or(&-1)
        } else {
            *self.stack_out.last().unwrap_or(&-1)
        }
    }

    pub(crate) fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}