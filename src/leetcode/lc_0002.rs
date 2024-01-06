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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if (l1 == None) && (l2 == None) {
        return None
    }
    if l1 == None {
        return l2;
    }
    if l2 == None {
        return l1;
    }        
    let (l1, l2) = (l1.unwrap(), l2.unwrap());
    let bruh = l1.val + l2.val;
    if bruh > 9 {
        let digit = bruh % 10;
        let carry = bruh / 10;
        return Some(Box::new(ListNode {
                                  val: digit,
                                  next: add_two_numbers(add_two_numbers(l1.next, l2.next), Some(Box::new(ListNode::new(carry))))
                                 })) 
        
    } else {
        let digit = bruh % 10;
        return Some(Box::new(ListNode {
                                     val: digit,
                                     next: add_two_numbers(l1.next, l2.next)
                                    }))
    }      
}