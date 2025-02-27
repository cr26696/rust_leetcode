
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
pub fn is_palindrome(mut l: Option<Box<ListNode>>) -> bool {
    let mut v = vec![];
    while let Some(n) = l.take() {
        v.push(n.val);
        l = n.next;
    }

    let x = v.iter();
    let y = v.iter().rev();
    x.zip(y).all(|(a, b)| a == b)
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
