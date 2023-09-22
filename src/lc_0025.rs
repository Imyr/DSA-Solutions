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





// fn reverse_list(mut head: Option<Box<ListNode>>, k: i32) -> Result<(Option<Box<ListNode>>, Option<Box<ListNode>>), ()> {
//     let mut master = None;
//     let mut i = 0;
//     while i<k {
//         if head.is_none() {
//             return Err(())
//         }
//         let mut node = head.unwrap();
//         head = node.next.take();
//         node.next = master;
//         master = Some(node);
//         i+=1;
//     }
//     Ok((master, head))
// }

// pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
//     let mut bruh = vec![];
    
//     loop {
//         let head_bkp = head.clone();
//         match reverse_list(head, k) {
//             Ok((reversed_nodes, head)) => {
                
//             },
//             Err() => {
//                 bruh.push(head);
//                 break;
//             },
//         };
//     }
// }

// pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
//     let k = k as usize;
//     let mut lmao = vec![];
//     while head.is_some() {
//         let mut hmm: Vec<Box<ListNode>> = vec![];
//         let mut i = 0;
//         while head.is_some() && i<k {
//             let mut lol = head.unwrap();
//             head = lol.next.take();
//             hmm.push(lol);
//             i+=1;
//         }   
//         if hmm.len()==k {
//             hmm.reverse();
//             for i in 0..k {
//                 lmao.push(Some(hmm[i].clone()));
//             }
//         } else {
//             for i in 0..hmm.len() {
//                 lmao.push(Some(hmm[i].clone()));
//             }
//         }
//     }
//     lmao.reverse();
//     let mut ll = None;
//     for i in lmao {
//         let mut bruh = i.unwrap();
//         bruh.next = ll;
//         ll = Some(bruh);
//     }
//     ll
// }