use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    dfs(&root).0
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
    if node.is_none() {
        return (true, 0);
    }
    let node = node.as_ref().unwrap().borrow();
    let node_left = dfs(&node.left);
    let node_right = dfs(&node.right);
    if node_left.0 && node_right.0 {
        if (node_right.1 - node_left.1).abs() <= 1 {
            return (true, node_left.1.max(node_right.1) + 1);
        }
    }
    (false, 0)
}