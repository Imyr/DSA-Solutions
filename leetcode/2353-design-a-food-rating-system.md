# 2353. Design a Food Rating System
- [Submission](https://leetcode.com/submissions/detail/1121682526/)

| Language | Runtime | Memory |
| :-       |       -:|      -:|
| Rust | 1051 ms | 20.8 MB |
```
use std::collections::HashMap;

struct FoodRatings {
    cuisine_food: HashMap<String, Vec<String>>,
    food_rating: HashMap<String, i32>,
    f_c: HashMap<String, String>
}

impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut cuisine_food: HashMap<String, Vec<String>> = HashMap::new();
        let mut food_rating: HashMap<String, i32> = HashMap::new();
        let mut f_c: HashMap<String, String> = HashMap::new();

        for i in 0..foods.len() {
            food_rating.insert(foods[i].clone(), ratings[i]);
            f_c.insert(foods[i].clone(), cuisines[i].clone());
            let ins = cuisine_food.entry(cuisines[i].clone()).or_insert(vec![]);
            match ins.binary_search_by_key(&ratings[i], |x| *food_rating.get(x).unwrap()) {
                Err(idx) => {ins.insert(idx, foods[i].clone())},
                Ok(_) => {
                    let l = ins.partition_point(|x| *food_rating.get(x).unwrap() < ratings[i]);
                    let r = ins.partition_point(|x| !(*food_rating.get(x).unwrap() > ratings[i]));
                    ins.insert(l+ins[l..r].to_vec().binary_search(&foods[i]).err().unwrap(), foods[i].clone())
                },
            }
        }
        FoodRatings { cuisine_food, food_rating, f_c}
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        *self.food_rating.get_mut(&food).unwrap() = new_rating;
        let c = self.f_c.get(&food).unwrap();
        
        let v = self.cuisine_food.get_mut(c).unwrap();
        let i = v.iter().position(|x| x == &food).unwrap();
        v.remove(i);

        let z = self.cuisine_food.get(c).unwrap();
        let l = z.partition_point(|x| *self.food_rating.get(x).unwrap() < new_rating);
        let r = z.partition_point(|x| !(*self.food_rating.get(x).unwrap() > new_rating));
        let y = self.cuisine_food.get_mut(c).unwrap();
        y.insert(l+y[l..r].to_vec().binary_search(&food).err().unwrap(), food);
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        match self.cuisine_food.get(&cuisine) {
            None => panic!(),
            Some(ins) => {
                ins[ins.partition_point(
                    |x| self.food_rating.get(x).unwrap() < self.food_rating.get(ins.last().unwrap()).unwrap()
                )].clone()
            },
        }
    }
}
```
