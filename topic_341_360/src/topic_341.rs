
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

pub struct NestedIterator(Vec<i32>);

impl NestedIterator {

    pub fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut init = vec![];
        nested_list.into_iter().rev().for_each(|x| {
            match x {
                NestedInteger::Int(val) => init.push(val),
                NestedInteger::List(vec) =>
                    to_vec(vec).into_iter().for_each(|val| init.push(val))
            }
        });
        NestedIterator(init)
    }

    pub fn next(&mut self) -> i32 {
        self.0.pop().unwrap()
    }

    pub fn has_next(&self) -> bool {
        !self.0.is_empty()
    }
}

fn to_vec(x: Vec<NestedInteger>) -> Vec<i32> {
    let mut ans = vec![];
    x.into_iter().rev().for_each(|i| {
        match i {
            NestedInteger::Int(val) => ans.push(val),
            NestedInteger::List(vec) => {
                to_vec(vec).into_iter().for_each(|val| ans.push(val));
            }
        }
    });
    ans
}