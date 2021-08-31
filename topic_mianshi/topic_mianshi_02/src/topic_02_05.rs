use crate::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let (mut p, mut q, mut carry) = (l1, l2, 0);
    let mut current = head.as_mut();
    while p.is_some() || q.is_some() {
        let mut sum  = carry;
        if let Some(v) = p {
            sum += v.val;
            p = v.next;
        }
        if let Some(v) = q {
            sum += v.val;
            q = v.next;
        }
        carry = sum / 10;
        if let Some(cur) = current {
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            current = cur.next.as_mut();
        }
    }
    if carry > 0 {
        current.unwrap().next = Some(Box::new(ListNode::new(1)));
    }
    head.unwrap().next
}