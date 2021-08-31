use std::cell::RefCell;
use std::rc::Rc;
use crate::TreeNode;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 0 {
        return None;
    }
    dfs(nums)
}

fn dfs(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.len() == 1 {
        return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
    }
    let head_index = nums.len() / 2;
    let mut head = TreeNode::new(nums[head_index]);
    head.left = dfs(nums[0..head_index].to_vec());
    if head_index < nums.len() - 1 {
        head.right = dfs(nums[head_index + 1..].to_vec());
    }
    Some(Rc::new(RefCell::new(head)))
}