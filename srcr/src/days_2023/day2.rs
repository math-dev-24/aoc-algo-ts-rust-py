use std::collections::HashMap;
use regex::Regex;

pub fn solve_day2_2023(input: &str) -> u32 {
    let games: Vec<&str> = input.lines().collect();
    let mut total_valid_ids: u32 = 0;
    let mut total_power: u32 = 0;

    let max_cubes: HashMap<&str, u32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ].iter().cloned().collect();

    let re: Regex = Regex::new(r"(\d+)\s+(\w+)").unwrap();

    for (i, game) in games.iter().enumerate() {
        let bags = game.split(":").collect::<Vec<&str>>()[1]
            .split(";").collect::<Vec<&str>>();

        let game_id = (i + 1) as u32;
        let mut valid_game: bool = true;

        let mut required_cubes: HashMap<&str, u32> = [
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ].iter().cloned().collect();

        for bag in bags {
            for cap in re.captures_iter(bag) {
                let quantity: u32 = cap[1].parse().unwrap();
                let color: String = cap[2].to_string();

                if let Some(current_max) = required_cubes.get_mut(color.as_str()) {
                    *current_max = (*current_max).max(quantity);
                }

                if let Some(&max_allowed) = max_cubes.get(color.as_str()) {
                    if quantity > max_allowed {
                        valid_game = false;
                    }
                }
            }
        }

        if valid_game {
            total_valid_ids += game_id;
        }

        let power = required_cubes["red"]
            * required_cubes["green"]
            * required_cubes["blue"];

        total_power += power;
    }

    println!("Jeux valides (Partie 1) : {}", total_valid_ids);
    println!("Somme des puissances (Partie 2) : {}", total_power);

    total_power
}