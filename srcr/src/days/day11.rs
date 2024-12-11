use std::collections::HashMap;

pub fn solve_day_11(input: &str) -> u128 {
    let mut stone_counts: HashMap<u128, u128> = HashMap::new();
    let click = 75;

    for stone in input.trim().split_whitespace().map(|s| s.parse::<u128>().unwrap()) {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..click {
        let mut new_stone_counts: HashMap<u128, u128> = HashMap::new();

        for (&stone, &count) in &stone_counts {
            if stone == 0 {
                *new_stone_counts.entry(1).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_str = stone.to_string();
                let half = stone_str.len() / 2;
                let left = stone_str[..half].parse::<u128>().unwrap();
                let right = stone_str[half..].parse::<u128>().unwrap();

                *new_stone_counts.entry(left).or_insert(0) += count;
                *new_stone_counts.entry(right).or_insert(0) += count;
            } else {
                // Multiplier par 2024
                *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stone_counts = new_stone_counts;
    }
    println!("Nombre de pierres après {} clignements : {}", click, stone_counts.values().sum::<u128>());
    stone_counts.values().sum()
}