use std::collections::HashMap;

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut directions: HashMap<String, Vec<String>> = HashMap::new();
    for path in paths {
        match directions.get_mut(&path[0]) {
            Some(x) => {x.push(path[1].clone());},
            None => {directions.insert(path[0].clone(), vec![path[1].clone()]);},
        }
        if let None = directions.get_mut(&path[1]) {
            directions.insert(path[1].clone(), vec![]);
        }
    }
    for (s, d) in directions {
        if d.len() == 0 {
            return s
        }
    }
    unreachable!()
}