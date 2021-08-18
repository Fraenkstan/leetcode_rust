use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>)
                  -> bool {
    dfs1(&root, &sub_root)
}

fn dfs1(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    if dfs2(root, sub_root) {
        return true
    }
    match root {
        Some(root_node) => {
            let root_ref = root_node.borrow();
            dfs2(root, sub_root) || dfs1(&root_ref.left, sub_root) || dfs1(&root_ref.right, sub_root)
        }
        None => false
    }
}

fn dfs2(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root, sub_root) {
        (Some(root_node), Some(sub_root_node)) => {
            let (root_ref, sub_root_ref) =
                (root_node.borrow(), sub_root_node.borrow());
            root_ref.val == sub_root_ref.val && dfs2(&root_ref.left, &sub_root_ref.left) &&
                dfs2(&root_ref.right, &sub_root_ref.right)
        }
        (None, None) => true,
        _ => false
    }
}