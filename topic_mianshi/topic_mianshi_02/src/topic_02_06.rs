use crate::ListNode;

pub(crate) fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    dfs(&head, &mut &head)
}

fn dfs(cur_node: &Option<Box<ListNode>>, fronter: &mut &Option<Box<ListNode>>) -> bool {
    if cur_node.is_some() {
        if !dfs(&cur_node.as_ref().unwrap().next, fronter) {
            return false;
        }
        if &cur_node.as_ref().unwrap().val != &fronter.as_ref().unwrap().val {
            return false;
        }
        // 修改 fronter 的值
        *fronter = &mut &fronter.as_ref().unwrap().next;
    }
    true
}
