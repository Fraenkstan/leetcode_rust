use std::collections::VecDeque;

pub struct AnimalShelf {
    cat_deque: VecDeque<i32>,
    dog_deque: VecDeque<i32>,
}

impl AnimalShelf {
    pub(crate) fn new() -> Self {
        AnimalShelf {
            cat_deque: VecDeque::new(),
            dog_deque: VecDeque::new(),
        }
    }

    pub(crate) fn enqueue(&mut self, animal: Vec<i32>) {
        if animal[1] == 0 {
            self.cat_deque.push_back(animal[0]);
        } else {
            self.dog_deque.push_back(animal[0]);
        }
    }

    pub(crate) fn dequeue_any(&mut self) -> Vec<i32> {
        if self.dog_deque.is_empty() && self.cat_deque.is_empty() {
            return vec![-1, -1];
        }
        return if self.cat_deque.front().or(Some(&i32::MAX))
            < self.dog_deque.front().or(Some(&i32::MAX))
        {
            vec![self.cat_deque.pop_front().unwrap(), 0]
        } else {
            vec![self.dog_deque.pop_front().unwrap(), 1]
        };
    }

    pub(crate) fn dequeue_dog(&mut self) -> Vec<i32> {
        if self.dog_deque.is_empty() {
            return vec![-1, -1];
        }
        return vec![self.dog_deque.pop_front().unwrap(), 1];
    }

    pub(crate) fn dequeue_cat(&mut self) -> Vec<i32> {
        if self.cat_deque.is_empty() {
            return vec![-1, -1];
        }
        return vec![self.cat_deque.pop_front().unwrap(), 0];
    }
}
