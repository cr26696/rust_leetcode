// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    // 从向量创建链表
    // 向量为空时panic!
    #[allow(dead_code)]
    pub fn from_vec(v: Vec<i32>) -> Box<ListNode> {
        if v.is_empty() {
            panic!("创建链表时，向量不能为空！");
        }
        let mut head = Box::new(ListNode::new(v[0])); // 创建头节点
        let mut current = &mut head; // 当前节点指向头节点

        for &value in &v[1..] {
            // 遍历向量的其余部分
            current.next = Some(Box::new(ListNode::new(value))); // 创建新节点并链接
            current = current.next.as_mut().unwrap(); // 移动到下一个节点
        }

        head // 返回链表的头节点
    }
    #[allow(dead_code)]
    pub fn log(&self) {
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