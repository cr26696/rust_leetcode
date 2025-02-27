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
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut bl1 = l1.as_mut().unwrap();
    let mut bl2 = l2.as_mut().unwrap();
    let mut b_head = Box::new(ListNode::new(0));
    let mut b_cur: &mut ListNode = b_head.as_mut();
    let mut inc = 0;
    loop {
        let sum;
        let (a, b) = (bl1.val, bl2.val);
        print!("{}+{}+{}=", a, b, inc);
        (inc, sum) = full_adder(inc, a, b);
        println!("{}{}", inc, sum);
        b_cur.next = Some(Box::new(ListNode::new(sum)));
        b_cur = b_cur.next.as_mut().unwrap();
        if bl1.next.is_none() && bl2.next.is_none() {
            //计算最后及可能的
            if inc == 1 {
                b_cur.next = Some(Box::new(ListNode::new(inc)));
            }
            break;
        } else if bl1.next.is_none() {
            bl1.val = 0;
            bl2 = bl2.next.as_mut().unwrap();
        } else if bl2.next.is_none() {
            bl2.val = 0;
            bl1 = bl1.next.as_mut().unwrap();
        } else {
            bl1 = bl1.next.as_mut().unwrap();
            bl2 = bl2.next.as_mut().unwrap();
        }
    }

    return b_head.next;
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
