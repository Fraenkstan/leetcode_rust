use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

mod topic_04_01;
mod topic_04_02;
mod topic_04_03;
mod topic_04_04;
mod topic_04_05;
mod topic_04_09;
mod topic_04_10;
mod topic_04_12;

#[cfg(test)]
mod tests {
    use crate::TreeNode;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::topic_04_01::find_whether_exists_path;
    use crate::topic_04_02::sorted_array_to_bst;
    use crate::topic_04_03::list_of_depth;
    use crate::topic_04_04::is_balanced;
    use crate::topic_04_05::is_valid_bst;
    use crate::topic_04_09::bst_sequences;
    use crate::topic_04_10::check_sub_tree;
    use crate::topic_04_12::path_sum;

    #[test]
    fn solution_04_01() {
        println!("{}", find_whether_exists_path(3, vec![vec![0,1], vec![0,2],
                                                        vec![1,2], vec![1,2]], 0, 2));
        println!("{}", find_whether_exists_path(5, vec![vec![0,1], vec![0,2],
                                                        vec![0,4], vec![0,4], vec![0,1],
                                                        vec![1,3], vec![1,4], vec![1,3],
                                                        vec![2,3], vec![3,4]], 0, 4));
    }

    #[test]
    fn solution_04_02() {
        println!("{:?}", sorted_array_to_bst(vec![-10,-3,0,5,9]));
    }

    #[test]
    fn solution_04_03() {
        let mut head = TreeNode::new(1);
        let mut left1 = TreeNode::new(2);
        let mut right1 = TreeNode::new(3);
        let mut left2 = TreeNode::new(4);
        let right2 = TreeNode::new(5);
        let right3 = TreeNode::new(7);
        let left3 = TreeNode::new(8);
        left2.left = Some(Rc::new(RefCell::new(left3)));
        left1.left = Some(Rc::new(RefCell::new(left2)));
        left1.right = Some(Rc::new(RefCell::new(right2)));
        right1.right = Some(Rc::new(RefCell::new(right3)));
        head.left = Some(Rc::new(RefCell::new(left1)));
        head.right = Some(Rc::new(RefCell::new(right1)));
        println!("{:?}", list_of_depth(Some(Rc::new(RefCell::new(head)))));
    }

    #[test]
    fn solution_04_04() {
        let mut root = TreeNode::new(3);
        let left1 = TreeNode::new(9);
        let mut right1 = TreeNode::new(20);
        let left2 = TreeNode::new(15);
        let right2 = TreeNode::new(7);
        right1.left = Some(Rc::new(RefCell::new(left2)));
        right1.right = Some(Rc::new(RefCell::new(right2)));
        root.left = Some(Rc::new(RefCell::new(left1)));
        root.right = Some(Rc::new(RefCell::new(right1)));
        println!("{}", is_balanced(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn solution_04_05() {
        let mut root = TreeNode::new(2);
        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(node1)));
        root.right = Some(Rc::new(RefCell::new(node2)));
        println!("{}", is_valid_bst(Some(Rc::new(RefCell::new(root)))));

        let mut root = TreeNode::new(5);
        let node1 = TreeNode::new(1);
        root.left = Some(Rc::new(RefCell::new(node1)));
        let mut node2 = TreeNode::new(4);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(6);
        node2.left = Some(Rc::new(RefCell::new(node3)));
        node2.right = Some(Rc::new(RefCell::new(node4)));
        root.right = Some(Rc::new(RefCell::new(node2)));
        println!("{}", is_valid_bst(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn solution_04_09() {
        let mut root = TreeNode::new(2);
        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(3);
        root.left = Some(Rc::new(RefCell::new(node1)));
        root.right = Some(Rc::new(RefCell::new(node2)));
        println!("{:?}", bst_sequences(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn solution_04_10() {
        let mut root1 = TreeNode::new(1);
        let node1 = TreeNode::new(2);
        let node2 = TreeNode::new(3);
        root1.left = Some(Rc::new(RefCell::new(node1)));
        root1.right = Some(Rc::new(RefCell::new(node2)));

        let root2 = TreeNode::new(2);
        println!("{}", check_sub_tree(Some(Rc::new(RefCell::new(root1))),
                                      Some(Rc::new(RefCell::new(root2)))));
    }

    #[test]
    fn solution_04_12() {
        let mut root = TreeNode::new(5);
        let mut node1 = TreeNode::new(4);
        let mut node2 = TreeNode::new(8);
        let mut node3 = TreeNode::new(11);
        let node4 = TreeNode::new(13);
        let mut node5 = TreeNode::new(4);
        let node6 = TreeNode::new(7);
        let node7 = TreeNode::new(2);
        let node8 = TreeNode::new(5);
        let node9 = TreeNode::new(1);
        node3.left = Some(Rc::new(RefCell::new(node6)));
        node3.right = Some(Rc::new(RefCell::new(node7)));
        node5.left = Some(Rc::new(RefCell::new(node8)));
        node5.right = Some(Rc::new(RefCell::new(node9)));
        node1.left = Some(Rc::new(RefCell::new(node3)));
        node2.left = Some(Rc::new(RefCell::new(node4)));
        node2.right = Some(Rc::new(RefCell::new(node5)));
        root.left = Some(Rc::new(RefCell::new(node1)));
        root.right = Some(Rc::new(RefCell::new(node2)));
        println!("{}", path_sum(Some(Rc::new(RefCell::new(root))), 22));
    }
}
