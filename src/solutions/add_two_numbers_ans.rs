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
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: add_two_numbers(
                l1.and_then(|x| {carry += x.val; x.next}), 
                l2.and_then(|x| {carry += x.val; x.next}), 
                carry / 10
            ),
            val: carry % 10
        }))
    }
}

fn full_adder(inc: i32, n1: i32, n2: i32) -> (i32, i32) {
    let sum = n1 + n2 + inc;
    return (sum / 10, sum % 10);
}

fn main() {
    let input1 = vec![1, 2, 4, 1, 6];
    let input2 = vec![1, 3, 4, 1, 4];

    let mut head1 = ListNode::from_vec(input1);
    let mut head2 = ListNode::from_vec(input2);
    // head1.clone().unwrap().log();
    let r = add_two_numbers(head1, head2);
    r.unwrap().log();
}
