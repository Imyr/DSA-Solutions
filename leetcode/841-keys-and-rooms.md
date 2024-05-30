# 841. Keys and Rooms
- [Submission](https://leetcode.com/submissions/detail/1248517580/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1 ms | 2.2 MB |
```
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut stack = vec![0];
        while let Some(room) = stack.pop() {
            if visited[room] { continue; }
            visited[room] = true;
            for i in rooms[room].iter().map(|x| *x as usize) { stack.push(i) }
        }
        visited.len() == visited.into_iter().filter(|x| *x).count()
    }
}
```
