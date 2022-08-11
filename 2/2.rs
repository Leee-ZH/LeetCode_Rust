// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut curr = &mut dummy;
    let mut state = (l1, l2, 0, 0);
    // l1, l2, sum, offset

    loop {
        state = match state{
            (None, None, _, 0) => break,
            (None, None, _, offset) => (None, None, offset, 0),
            (Some(node), None, _, offset) | (None, Some(node), _, offset) => (node.next, None, (node.val + offset) % 10, (node.val + offset) / 10),
            (Some(node1), Some(node2), _, offset) => (node1.next, node2.next, (node1.val + node2.val + offset) % 10, (node1.val + node2.val + offset) / 10)
        };
        *curr = Some(Box::new(ListNode::new(state.2)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    return dummy;
}

fn main() {
    println!("??");
}
