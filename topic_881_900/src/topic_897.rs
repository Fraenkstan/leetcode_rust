use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
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

#[allow(unused)]
pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue = VecDeque::new();
    let mut ans = None;
    let mut root = root;
    while root.is_some() {
        while let Some(node) = root.take() {
            root = node.borrow_mut().right.take();
            queue.push_back(node);
        }
        while !root.is_some() && queue.len() > 0 {
            if let Some(x) = queue.pop_back() {
                root = x.borrow_mut().left.take();
                x.borrow_mut().right = ans;
                ans = Some(x);
            }
        }
    }
    ans
}