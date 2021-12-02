
mod topic_665;
mod topic_671;
mod topic_678;

#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::topic_665::check_possibility;
    use crate::topic_671::{TreeNode, find_second_minimum_value};
    use crate::topic_678::check_valid_string;

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
        right1.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        right1.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.right = Some(Rc::new(RefCell::new(right1)));
        println!("{}", find_second_minimum_value(Some(Rc::new(RefCell::new(root)))));
    }

    #[test]
    fn solution_678() {
        println!("{}", check_valid_string("()".to_string()));
        println!("{}", check_valid_string("(*))".to_string()));
        println!("{}", check_valid_string("((*)".to_string()));
        println!("{}", check_valid_string("(*)*)".to_string()));
        println!("{}", check_valid_string("(**))))".to_string()));
        println!("{}", check_valid_string("((((()(()()()*()(((((*)()*(**(())))))(())()())((\
        (())())())))))))(((((())*)))()))(()((*()*(*)))(*)()".to_string()));
        println!("{}", check_valid_string("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)\
        ))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".to_string()));
    }
}
