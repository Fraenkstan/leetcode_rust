use std::rc::Rc;
use std::cell::RefCell;
use crate::{TreeNode, ListNode};
use std::collections::BTreeMap;

pub fn list_of_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Box<ListNode>>> {
    let mut map = BTreeMap::new();
    dfs(&tree, &mut map, 0);
    map.iter().map(|(_k, v)| {
        let mut head = ListNode::new(v[0]);
        let mut current = &mut head;
        let mut i = 1;
        while i < v.len() {
            let node = ListNode::new(v[i]);
            current.next = Some(Box::new(node));
            current = current.next.as_mut().unwrap();
            i += 1;
        }
        Some(Box::new(head))
    }).collect()
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, map: &mut BTreeMap<i32, Vec<i32>>, degree: i32) {
    if node.is_none() {
        return;
    }
    let node = node.as_ref().unwrap().borrow();
    map.entry(degree).or_insert(vec![]).push(node.val);
    dfs(&node.left, map, degree + 1);
    dfs(&node.right, map, degree + 1);
}