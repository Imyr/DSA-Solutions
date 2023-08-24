#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len: u8 = 0;
    let mut iter = head.clone();
    while head!=None {
        head = head.unwrap().next;
        len+=1;
    }
    let mut values = vec![];
    for _ in 1..=len-n as u8{
        let bruh = iter.unwrap();
        values.push(bruh.val as u8);
        iter = bruh.next;
    }
    iter = match iter {
        Some(node) => node.next,
        None => None,
    };
    values.reverse();
    for value in values {
        iter = Some(Box::new(ListNode{val: value as i32, next: iter}));
    }
    iter
}