use crate::ListNode;

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut le = ListNode { val: 0, next: None };
    let mut gt = ListNode { val: 0, next: None };

    let mut le_ptr = &mut le;
    let mut gt_ptr = &mut gt;
    let mut head = head;

    while let Some(mut node) = head {
        let next = node.next.take();
        if node.val < x {
            le_ptr.next = Some(node);
            le_ptr = le_ptr.next.as_mut().unwrap().as_mut();
        } else {
            gt_ptr.next = Some(node);
            gt_ptr = gt_ptr.next.as_mut().unwrap().as_mut();
        }
        head = next;
    }
    le_ptr.next = gt.next.take();
    le.next
}
