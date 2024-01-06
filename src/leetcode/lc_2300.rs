pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();
    spells.iter().map(|&s| {
        (potions.len() - potions.partition_point(|&p| (s as i64*p as i64) < success)) as i32
    }).collect()
}