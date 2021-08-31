use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack= vec![];
    let mut cur= root;
    let mut prev :Option<i32> = Option::None;
    while !stack.is_empty() || cur.is_some() {
        while cur.is_some() {
            let cur_node = cur.unwrap();
            stack.push(cur_node.clone());
            if cur_node.borrow().left.is_some() {
                cur = cur_node.borrow().left.clone();
            } else {
                break;
            }
        }
        let cur_node = stack.pop().unwrap();
        match prev {
            Some(prev_num) => {
                if prev_num >= cur_node.borrow().val {
                    return false;
                }
                prev = Option::Some(cur_node.borrow().val);
            }
            None => prev = Option::Some(cur_node.borrow().val)
        }
        cur = cur_node.borrow().right.clone();
    }
    true
}