
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

fn main() {
    let input = vec![1,2,3,4,5];

    let mut head :ListNode = ListNode::new(input[0]);
    for (i,&e) in input.iter().enumerate() {
        if i == 0 {continue;}
        head.push(e);
    }
    head.log();
    let r = reverse_list(Some(Box::new(head)));
    r.unwrap().log();
    // println!("{:?}", r);
}
