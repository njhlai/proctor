#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[rustfmt::skip]
    #[must_use]
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    /// Constructs a `ListNode` from `input`.
    #[must_use]
    pub fn from(input: Vec<i32>) -> Option<Box<Self>> {
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut cur = res.as_mut();

        for n in input {
            if let Some(node) = cur {
                node.next = Some(Box::new(ListNode::new(n)));
                cur = node.next.as_mut();
            }
        }

        res?.next
    }
}
