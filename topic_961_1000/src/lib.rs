#![feature(refcell_take)]

mod topic_981;
mod topic_987;
mod topic_994;

#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::topic_981::TimeMap;
    use crate::topic_987::{TreeNode, vertical_traversal};
    use crate::topic_994::oranges_rotting;

    #[test]
    fn solution_981() {
        let mut test = TimeMap::new();
        test.set("foo".to_string(), "bar".to_string(), 1);
        println!("{}", test.get("foo".to_string(), 1));
        println!("{}", test.get("foo".to_string(), 3));
        test.set("foo".to_string(), "bar2".to_string(), 4);
        println!("{}", test.get("foo".to_string(), 4));
        println!("{}", test.get("foo".to_string(), 5));
    }

    #[test]
    fn solution_987() {
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);
        let node6 = TreeNode::new(6);
        let node7 = TreeNode::new(7);
        node2.left = Some(Rc::new(RefCell::new(node4)));
        node2.right = Some(Rc::new(RefCell::new(node5)));
        node3.left = Some(Rc::new(RefCell::new(node6)));
        node3.right = Some(Rc::new(RefCell::new(node7)));
        root.left = Some(Rc::new(RefCell::new(node2)));
        root.right = Some(Rc::new(RefCell::new(node3)));
        println!("{:?}", vertical_traversal(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn solution_994() {
        println!("{}", oranges_rotting(vec![vec![2,1,1], vec![1,1,0], vec![0,1,1]]));
        println!("{}", oranges_rotting(vec![vec![2,1,1], vec![0,1,1], vec![1,0,1]]));
        println!("{}", oranges_rotting(vec![vec![0,2]]));
    }
}
