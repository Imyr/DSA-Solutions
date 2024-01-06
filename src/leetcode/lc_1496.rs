use std::collections::HashSet;

pub fn is_path_crossing(path: String) -> bool {
    let mut points = HashSet::new();
    let mut now = (0, 0);
    points.insert(now);
    for c in path.chars() {
        println!("{:?}", now);
        match c {
            'N' => now.1 += 1,
            'S' => now.1 -= 1,
            'E' => now.0 += 1,
            'W' => now.0 -= 1,
            _ => unreachable!()
        }
        match points.insert(now) {
            false => return true,
            true => continue
        }
    }
    false       
}