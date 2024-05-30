# 950. Reveal Cards In Increasing Order
- [Submission](https://leetcode.com/submissions/detail/1228902600/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.1 MB |
```
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();
        let mut my_deck = std::collections::VecDeque::new();
        my_deck.push_front(deck[deck.len()-1]);

        for i in (0..deck.len()-1).rev() {
            let last = my_deck.pop_back().unwrap();
            my_deck.push_front(last);
            my_deck.push_front(deck[i]);
        } 
        my_deck.into()
    }
}
```
