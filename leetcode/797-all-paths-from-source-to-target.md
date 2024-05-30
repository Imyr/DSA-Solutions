# 797. All Paths From Source to Target
- [Submission](https://leetcode.com/submissions/detail/1251665856/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 7 ms | 2.9 MB |
```
impl Solution {
    fn dfs(source: i32, final_destination: i32, graphs: &Vec<Vec<i32>>, mut current_path: Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        current_path.push(source);
        if source == final_destination { paths.push(current_path); return; }
        for node in graphs[source as usize].iter() {
            Solution::dfs(*node, final_destination, graphs, current_path.clone(), paths);
        }
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let dest = graph.len() as i32 - 1;
        let mut paths = vec![];
        Solution::dfs(0, dest,  &graph, vec![], &mut paths);
        paths
    }
}
```
