
#[derive(Debug)]
pub struct BinaryTreeNode<T>{

    val: T,

    children_num: usize,

    children: Vec<BinaryTreeNode<T>>
}

#[allow(dead_code)]
impl <T> BinaryTreeNode<T> {

    pub fn new(val: T) -> BinaryTreeNode<T> {
        BinaryTreeNode{
            val,
            children_num: 0,
            children: Vec::with_capacity(2)
        }
    }

    pub fn add_children(&mut self, children: BinaryTreeNode<T>) -> Result<(), &str> {
        if self.children_num >= 2 {
            Err("子节点已满")
        }
        else {
            self.children_num += 1;
            self.children.push(children);
            Ok(())
        }
    }
}