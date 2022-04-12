mod topic_661;
mod topic_665;
mod topic_671;
mod topic_678;
mod topic_680;

#[cfg(test)]
mod tests {

    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::topic_661::image_smoother;
    use crate::topic_665::check_possibility;
    use crate::topic_671::{find_second_minimum_value, TreeNode};
    use crate::topic_678::check_valid_string;
    use crate::topic_680::valid_palindrome;

    #[test]
    fn solution_661() {
        println!(
            "{:?}",
            image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
        );
        println!(
            "{:?}",
            image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ])
        );
    }

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
        println!(
            "{}",
            find_second_minimum_value(Some(Rc::new(RefCell::new(root))))
        );
    }

    #[test]
    fn solution_678() {
        println!("{}", check_valid_string("()".to_string()));
        println!("{}", check_valid_string("(*))".to_string()));
        println!("{}", check_valid_string("((*)".to_string()));
        println!("{}", check_valid_string("(*)*)".to_string()));
        println!("{}", check_valid_string("(**))))".to_string()));
        println!(
            "{}",
            check_valid_string(
                "((((()(()()()*()(((((*)()*(**(())))))(())()())((\
        (())())())))))))(((((())*)))()))(()((*()*(*)))(*)()"
                    .to_string()
            )
        );
        println!(
            "{}",
            check_valid_string(
                "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)\
        ))))))(())(()))())((*()()(((()((()*(())*(()**)()(())"
                    .to_string()
            )
        );
    }

    #[test]
    fn solution_680() {
        println!("{}", valid_palindrome("aba".to_string()));
        println!("{}", valid_palindrome("abca".to_string()));
        println!("{}", valid_palindrome("abc".to_string()));
        println!("{}", valid_palindrome("abcca".to_string()));
        println!("{}", valid_palindrome("accba".to_string()));
        println!("{}", valid_palindrome("aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga".to_string()));
    }
}
