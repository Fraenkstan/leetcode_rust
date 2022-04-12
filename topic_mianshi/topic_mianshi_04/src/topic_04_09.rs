use crate::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn bst_sequences(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    if let Some(root) = root {
        let mut queue = VecDeque::new();
        queue.push_back(root);
        dfs(&mut ans, &mut queue, &mut vec![]);
    } else {
        ans = vec![vec![]]
    }
    ans
}

fn dfs(ans: &mut Vec<Vec<i32>>, queue: &mut VecDeque<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>) {
    if queue.is_empty() {
        ans.push(path.clone());
        return;
    }
    for _i in 0..queue.len() {
        let node_node = queue.pop_front().unwrap();
        let node = node_node.as_ref().borrow();
        if node.left.is_some() {
            queue.push_back(node.left.as_ref().unwrap().clone());
        }
        if node.right.is_some() {
            queue.push_back(node.right.as_ref().unwrap().clone());
        }
        path.push(node.val);
        dfs(ans, queue, path);
        if node.left.is_some() {
            queue.pop_back();
        }
        if node.right.is_some() {
            queue.pop_back();
        }
        queue.push_back(node_node.clone());
        path.remove(path.len() - 1);
    }
}
