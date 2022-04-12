use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn check_sub_tree(
    t1: Option<Rc<RefCell<TreeNode>>>,
    t2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let vec1 = to_vec(t1);
    let vec2 = to_vec(t2);
    kmp(vec1, vec2)
}

fn to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    if let Some(root) = root {
        dfs(&mut ans, &root);
    }
    ans
}

fn dfs(ans: &mut Vec<i32>, node: &Rc<RefCell<TreeNode>>) {
    let node = node.borrow();
    if node.left.is_some() {
        dfs(ans, node.left.as_ref().unwrap());
    }
    ans.push(node.val);
    if node.right.is_some() {
        dfs(ans, node.right.as_ref().unwrap());
    }
}

// t1.len() > t2.len()
fn kmp(t1: Vec<i32>, t2: Vec<i32>) -> bool {
    let mut next = vec![0; t2.len()];
    for i in 1..next.len() {
        if t2[next[i - 1]] == t2[i] {
            next[i] = next[i - 1] + 1;
        }
    }
    let (mut i, mut j) = (0, 0);
    while i < t1.len() {
        if t1[i] == t2[j] {
            i += 1;
            j += 1;
        } else if j != 0 {
            j = next[j - 1];
        } else {
            i += 1;
        }
        if j == t2.len() {
            return true;
        }
    }
    false
}
