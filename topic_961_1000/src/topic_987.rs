use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{BTreeMap};

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


pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut map = BTreeMap::new();
    dfs(root.as_ref(), 0, 0, &mut map);
    map.into_iter().map(|(_k, mut v)| {
        v.sort_by(|&a, &b| {
            return if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else { a.0.cmp(&b.0) }
        });
        v.into_iter().map(|(_row, val)| val).collect()
    }).collect::<Vec<Vec<i32>>>()
}

fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, row: i32, col: i32, ans: &mut BTreeMap<i32, Vec<(i32, i32)>>) {
    match node {
        Some(node) => {
            let vec = ans.entry(col).or_insert(vec![]);
            let node = node.borrow();
            println!("{}", node.val);
            vec.push((row, node.val));
            dfs(node.left.as_ref(), row + 1, col - 1, ans);
            dfs(node.right.as_ref(), row + 1, col + 1, ans);
        },
        None => {}
    }
}
