mod topic_295;
mod topic_297;

#[cfg(test)]
mod tests {
    use crate::topic_295::MedianFinder;
    use crate::topic_297::{Codec, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn solution_295() {
        let mut test = MedianFinder::new();
        test.add_num(1);
        test.add_num(2);
        println!("{}", test.find_median());
        test.add_num(3);
        println!("{}", test.find_median());
    }

    #[test]
    fn solution_297() {
        let code_c = Codec::new();
        let mut root = TreeNode::new(1);
        let node_2 = TreeNode::new(2);
        let mut node_3 = TreeNode::new(3);
        let node_4 = TreeNode::new(4);
        let node_5 = TreeNode::new(5);
        node_3.left = Some(Rc::new(RefCell::new(node_4)));
        node_3.right = Some(Rc::new(RefCell::new(node_5)));
        root.left = Some(Rc::new(RefCell::new(node_2)));
        root.right = Some(Rc::new(RefCell::new(node_3)));
        println!("{}", code_c.serialize(Some(Rc::new(RefCell::new(root)))));
    }
}
