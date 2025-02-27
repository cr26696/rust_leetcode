
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
    fn push(&mut self, val: i32) {
        let mut node = self;
        loop {
            if node.next.is_none() {
                node.next = Some(Box::new(ListNode::new(val)));
                break;
            }
            node = node.next.as_deref_mut().unwrap();
        }
    }
}
fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut now = head.clone();
    let mut len = 0;
    while let Some(node) = now {
        len+=1;
        now = node.next;
    }
    println!("len/2: {}", len/2);
    let mut template: Vec<i32> = Vec::with_capacity(len/2);
    now = head.clone();
    //前半
    for _ in 0..len/2 {
        let node = now.unwrap();
        now = node.next;
        template.push(node.val);

    }
    if len % 2 == 1 {
        now = now.unwrap().next;
    }
    //后半
    for i in (0..len/2).rev() {
        let node = now.unwrap();
        now = node.next;
        if template[i] != node.val {
            return false;
        }
    }
    true
}

fn main() {
    let input = vec![1,2,3,2,1,1,2,3,2,1,2];

    let mut head :ListNode = ListNode::new(input[0]);
    for (i,&e) in input.iter().enumerate() {
        if i == 0 {continue;}
        head.push(e);
    }
    head.log();
    let r = is_palindrome(Some(Box::new(head)));
    // r.unwrap().log();
    println!("{:?}", r);
}
