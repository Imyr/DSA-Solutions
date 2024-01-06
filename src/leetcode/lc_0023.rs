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

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut hehe = vec![];
    let mut no_none = true;
    while no_none {
        no_none = false;
        let mut min = 10000;
        for i in 0..lists.len() {
            if lists[i]==None {
                continue;
            } else {
                no_none = true;
                let lul = lists[i].clone().unwrap();
                if lul.val<min {
                    min = lul.val;
                }
            }
        }
        for i in 0..lists.len() {
            if lists[i]==None {
                continue;
            } else {
                let lul = lists[i].clone().unwrap();
                if lul.val == min {
                    hehe.push(min);
                    lists[i] = lul.next;
                }
            }
        }
    }
    println!("{:?}", hehe);
    hehe.reverse();
    let mut ll = None;
    for i in hehe {
        ll = Some(Box::new(ListNode{next: ll, val: i,}))
    }
    ll
}