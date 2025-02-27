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

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur.next = list1.take();
                cur = cur.next.as_mut()?;
                list1 = cur.next.take();
            } else {
                cur.next = list2.take();
                cur = cur.next.as_mut()?;
                list2 = cur.next.take();
            }
        }
        cur.next = list1.or(list2);
        dummy.next
    }
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
