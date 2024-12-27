use std::collections::HashMap;
use regex::Regex;

pub fn solve_day2_2023(input: &str) -> (usize, usize) {
    let games: Vec<&str> = input.lines().collect();
    let mut total_valid_ids: usize = 0;
    let mut total_power: usize = 0;

    let max_cubes: HashMap<&str, u32> = [
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ].iter().cloned().collect();

    for (i, game) in games.iter().enumerate() {
        let bags = game.split(":").collect::<Vec<&str>>()[1]
            .split(";").collect::<Vec<&str>>();

        let game_id = i + 1;
        let mut valid_game: bool = true;

        let mut required_cubes: HashMap<&str, u32> = [
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ].iter().cloned().collect();

        for bag in bags {
            let re: Regex = Regex::new(r"(\d+)\s+(\w+)").unwrap();
            let mut observation: HashMap<String, u32> = HashMap::new();

            for cap in re.captures_iter(bag) {
                let quantity: u32 = cap[1].parse().unwrap();
                let color: String = cap[2].to_string();
                observation.insert(color.clone(), quantity);

                if let Some(current_max) = required_cubes.get_mut(color.as_str()) {
                    *current_max = (*current_max).max(quantity);
                }
            }

            for (color, &max_quantity) in &max_cubes {
                if let Some(&observed) = observation.get(&color.to_string()) {
                    if observed > max_quantity {
                        valid_game = false;
                        break;
                    }
                }
            }

            if !valid_game {
                break; // Si une observation dépasse les limites, le jeu est invalide
            }
        }

        // Partie 1 : Ajouter l'ID si le jeu est valide
        if valid_game {
            total_valid_ids += game_id;
        }

        // Partie 2 : Calculer la puissance après avoir traité toutes les observations
        let power = required_cubes["red"] as usize
            * required_cubes["green"] as usize
            * required_cubes["blue"] as usize;

        total_power += power;
    }

    println!("Total des IDs valides : {}", total_valid_ids);
    println!("Somme des puissances : {}", total_power);
    (total_valid_ids, total_power)
}