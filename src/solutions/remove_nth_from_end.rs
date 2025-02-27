use crate::listnode::ListNode;

#[allow(dead_code)]
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len = 1;
    let mut dummy = head.as_mut().unwrap();
    loop {
        if dummy.next.is_none() {
            break;
        }
        dummy = dummy.next.as_mut().unwrap();
        len += 1;
    }
    let del_idx = len - n;
    let prev_idx = 0.max(del_idx - 1);
    dummy = head.as_mut().unwrap();
    for _ in 1..del_idx {
        dummy = dummy.next.as_mut().unwrap();
    }
    if n == len {
        return head.unwrap().next;
    } else if n == 1 {
        dummy.next = None;
    } else {
        dummy.next = dummy.next.take().unwrap().next;
        println!("middle situ")
    }
    return head;
}
pub fn remove_nth_from_end2(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut l: *const _ = &head;
    let mut d = head.as_mut().unwrap();
    let mut i = 0;
    while let Some(sl) = unsafe { &*l } {
        l = &sl.next;
        if i > n {
            d = d.next.as_mut().unwrap()
        }
        i += 1;
    }
    if i == n {
        return head.unwrap().next;
    }
    d.next = d.next.take().unwrap().next;
    head
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![2];
        let head = ListNode::from_vec(nums);
        let n = 1;
        let r = remove_nth_from_end(Some(head), n);
        println!("{:?}",r);
        if r.is_some(){r.unwrap().log();}
    }
    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4, 5];
        let head = ListNode::from_vec(nums);
        let opt: Option<Box<ListNode>> = Some(head);
        let cloned = opt.clone();
        let cloned_box = &opt.as_ref().unwrap().clone();
        let singlenode = Some(Box::new(ListNode::new(5)));
        println!("{:p} 1", &opt);
        println!("{:p} 1", opt.as_ref().unwrap());
        println!("{:p} 2", *opt.as_ref().unwrap());
        println!("{:p} 2", opt.as_deref().unwrap());
        println!("{:p} 3 ", cloned.as_ref().unwrap());
        println!("{:p} 3* ", *cloned.as_ref().unwrap());
        println!("{:p} 3 ", cloned.as_deref().unwrap());
        println!("{:p} 4 ", cloned_box);
        println!("{:p} 5 ", *cloned_box);
        println!("{:p} 6 ", &singlenode.as_ref().unwrap().next);
        // println!("head {:p}",&head);
    }
}
