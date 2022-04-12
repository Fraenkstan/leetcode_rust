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

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut v = 0i32;
    fn add(r: &Option<Rc<RefCell<TreeNode>>>, v: &mut i32, is: bool) {
        let rb = r.as_ref().unwrap().borrow();
        match (&rb.left, &rb.right) {
            (Some(_), Some(_)) => {
                add(&rb.left, v, true);
                add(&rb.right, v, false);
            }
            (Some(_), None) => add(&rb.left, v, true),
            (None, Some(_)) => add(&rb.right, v, false),
            (None, None) => {
                if is {
                    *v += rb.val;
                }
            }
        }
    }
    add(&root, &mut v, false);
    v
}
