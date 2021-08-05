
mod topic_665;
mod topic_671;

#[cfg(test)]
mod tests {

    use crate::topic_665::check_possibility;
    use crate::topic_671::{TreeNode, find_second_minimum_value};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn solution_665() {
        let nums = vec![3, 4, 2, 5];
        println!("{}", check_possibility(nums));
        let nums = vec![5, 7, 1, 8];
        println!("{}", check_possibility(nums));
    }

    #[test]
    fn solution_671() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let mut right1 = TreeNode::new(5);
        root.right = Some(Rc::new(RefCell::new(right1)));
        right1.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        right1.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        println!("{}", find_second_minimum_value(Some(Rc::new(RefCell::new(root)))));
    }
}
