use crate::TreeNode;
use std::cell::RefCell;
use std::iter::once;
use std::rc::Rc;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    dfs(&root, vec![], sum)
}

#[allow(unused)]
fn count_path(path: &mut Vec<Vec<i32>>, node: &Rc<RefCell<TreeNode>>, mut vec: Vec<i32>) {
    let node = node.borrow();
    vec.push(node.val);
    match (&node.left, &node.right) {
        (None, None) => {
            path.push(vec.clone());
        }
        (Some(left), Some(right)) => {
            count_path(path, left, vec.clone());
            count_path(path, right, vec.clone());
        }
        (Some(left), None) => {
            count_path(path, left, vec.clone());
        }
        (None, Some(right)) => {
            count_path(path, right, vec.clone());
        }
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, s: Vec<i32>, sum: i32) -> i32 {
    if node.is_some() {
        let node = node.as_ref().unwrap().borrow();
        let s = s
            .iter()
            .map(|&i| i + node.val)
            .chain(once(node.val))
            .collect::<Vec<i32>>();
        println!("{:?}", s);
        return s.iter().filter(|&&i| i == sum).count() as i32
            + dfs(&node.left, s.clone(), sum)
            + dfs(&node.right, s, sum);
    }
    0
}
