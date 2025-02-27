use std::cmp::Ordering;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        if v.is_empty() {
            return None; // 如果向量为空，返回 None
        }

        let mut head = Box::new(ListNode::new(v[0])); // 创建头节点
        let mut current = &mut head; // 当前节点指向头节点

        for &value in &v[1..] {
            // 遍历向量的其余部分
            current.next = Some(Box::new(ListNode::new(value))); // 创建新节点并链接
            current = current.next.as_mut().unwrap(); // 移动到下一个节点
        }

        Some(head) // 返回链表的头节点
    }
    fn log(&self) {
        let mut node = self;
        loop {
            print!(" {}", node.val);
            match &node.next {
                Some(n) => {
                    node = n;
                }
                None => {
                    break;
                }
            }
        }
        println!()
    }
}
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    if list1.is_none() && list2.is_none() {
        return None;
    } else if list1.is_none() {
        return list2;
    } else if list2.is_none() {
        return list1;
    } else {
        head = Box::new(ListNode::new(0));
        let mut cur = &mut head;
        let mut node1 = &mut list1.clone().unwrap();
        let mut node2 = &mut list2.clone().unwrap();
        let mut val = i32::MIN;
        let mut flag_1_emp = false;
        let mut flag_2_emp = false;
        while !flag_1_emp && !flag_2_emp {
            if node1.val < node2.val {
                val = node1.val;
                cur.next = Some(Box::new(ListNode::new(val)));
                cur = cur.next.as_mut().unwrap();
                if node1.next.is_none() {
                    flag_1_emp = true;
                }else{
                    node1 = node1.next.as_mut().unwrap();
                }
            } else {
                val = node2.val;
                cur.next = Some(Box::new(ListNode::new(val)));
                cur = cur.next.as_mut().unwrap();
                if node2.next.is_none() {
                    flag_2_emp = true;
                }else{
                    node2 = node2.next.as_mut().unwrap();
                }
            }
        }
        if flag_1_emp{
            cur.next = Some(node2.clone());
        }else if flag_2_emp{
            cur.next = Some(node1.clone());
        }
    }
    head.next
}

fn main() {
    let input1 = vec![1, 2, 4, 98, 99];
    let input2 = vec![1, 3, 4, 95, 100];

    let mut head1 = ListNode::from_vec(input1);
    let mut head2 = ListNode::from_vec(input2);
    head1.clone().unwrap().log();
    let r = merge_two_lists(head1, head2);
    println!("{:?}", r);
    r.unwrap().log();
}
