use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct BSTIterator {
    expanded: Vec<i32>,
    counter: usize,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut v = vec![];
        expand_tree(&mut v, root);

        BSTIterator {
            expanded: v,
            counter: 0,
        }
    }

    fn next(&mut self) -> i32 {
        self.counter += 1;
        self.expanded[self.counter - 1]
    }

    fn has_next(&self) -> bool {
        self.expanded.len() > self.counter
    }
}

fn expand_tree(store: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(medium) = root {
        expand_tree(store, medium.borrow_mut().left.take());
        store.push(medium.borrow().val);
        expand_tree(store, medium.borrow_mut().right.take());
    }
}
