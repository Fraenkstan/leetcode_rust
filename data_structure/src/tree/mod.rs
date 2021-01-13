mod binary_tree;

#[derive(Debug, Clone)]
pub struct TreeNode<T> {

    val: T,

    degree: usize,

    pub children_num: usize,

    childrens: Vec<TreeNode<T>>,

    parent: Option<Box<TreeNode<T>>>,

}

impl <T> TreeNode<T> where T: Clone {

    pub fn new(val: T) -> TreeNode<T> {
        TreeNode{
            val,
            degree: 0,
            children_num: 0,
            childrens: Vec::new(),
            parent: None
        }
    }

    pub fn add_children_node(&mut self, mut node: TreeNode<T>) {
        node.parent = Some(Box::new(self.clone()));
        node.degree_add(self.degree + 1);
        self.childrens.push(node);
        self.children_num += 1;
    }

    fn degree_add(&mut self, length: usize) {
        for children in self.childrens.iter_mut() {
            children.degree_add(length);
        }
        self.degree += length;
    }

    pub fn add_children_val(&mut self, val: T) {
        self.children_num += 1;
        let mut children = TreeNode::new(val);
        children.degree = self.degree + 1;
        children.parent = Some(Box::new(self.clone()));
        self.childrens.push(children);
    }

    pub fn contains(&self, val: &T) -> bool where T: PartialEq {
        let mut result = false;
        if self.val == *val {
            result = true;
        }
        for child in self.childrens.iter() {
            if child.contains(val) {
                result = true;
                break;
            }
        }
        result
    }

    pub fn posit(&self, val: &T) -> Option<&TreeNode<T>> where T: PartialEq {
        if self.val == *val {
            return Some(self);
        }
        for child in self.childrens.iter() {
            let result = child.posit(val);
            match &result {
                None => {}
                Some(node) => {
                    return Some(node);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;

    #[test]
    fn tree_test() {
        let mut root = TreeNode::new(0);
        let mut children1 = TreeNode::new(1);
        let mut children2 = TreeNode::new(2);
        let mut children3 = TreeNode::new(3);

        children3.add_children_val(4);
        children2.add_children_node(children3);
        children1.add_children_node(children2);
        root.add_children_node(children1);
        root.add_children_val(5);

        println!("{:?}", root);
        println!("{}", root.contains(&0));
        println!("{}", root.contains(&4));
        println!("{}", root.contains(&6));

        println!("{:?}", root.posit(&5));
        // println!("{:?}", children1);
    }
}