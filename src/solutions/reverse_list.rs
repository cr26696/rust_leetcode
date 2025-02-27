use crate::listnode::ListNode;
#[allow(dead_code)]
fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {return head;}
    let mut node_head = head.unwrap();
    let mut next =  node_head.next.take();
    node_head.next = None;
    let mut out:Option<Box<ListNode>> = Some(node_head);
    while next.is_some() {
        let rest  = next.as_mut().unwrap().next.take();
        next.as_mut().unwrap().next = out;
        out = next; 
        next = rest;
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![1,2,3,4];
        let head = ListNode::from_vec(nums);
        head.log();
        let r = reverse_list(Some(head));
        r.unwrap().log();
    }
}
