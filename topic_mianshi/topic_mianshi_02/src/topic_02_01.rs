use crate::ListNode;

pub fn remove_duplicate_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some(mut head) = head {
        let mut vec = vec![];
        vec.push(head.val);
        let mut cur = &mut head;
        while let Some(next) = &cur.next {
            if vec.contains(&next.val) {
                cur.next = cur.next.take().unwrap().next;
            } else {
                vec.push(next.val);
                cur = cur.next.as_mut().unwrap();
            }
        }
        Some(head)
    } else {
        None
    }
}
