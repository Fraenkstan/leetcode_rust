use data_structure::rc_refcell_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut v = Vec::new();
    collect(root1, &mut v);
    let mut v2 = Vec::new();
    collect(root2, &mut v2);

    v == v2
}

fn collect(r: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    if r.is_none() {
        return;
    }

    let r = r.unwrap();
    let mut r = r.borrow_mut();
    if r.left.is_none() && r.right.is_none() {
        v.push(r.val);
        return;
    }

    collect(r.left.take(), v);
    collect(r.right.take(), v);
}
