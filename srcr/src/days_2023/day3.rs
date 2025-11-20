use std::collections::HashMap;

pub fn solve_day3_2023(input: &str) -> i32 {

    let lines: Vec<&str> = input.lines().collect();
    let number_of_lines = lines.len();
    let mut result = 0;

    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();

    let total_chars = grid.get(0).map_or(0, |row| row.len());

    for x in 0..number_of_lines {
        let mut y = 0;

        while y < total_chars {
            let item = grid[x][y];

            if item.is_digit(10) {
                let mut current_number_str = String::new();
                let start_y = y;

                let mut current_y = y;
                while current_y < total_chars && grid[x][current_y].is_digit(10) {
                    current_number_str.push(grid[x][current_y]);
                    current_y += 1;
                }

                let end_y = current_y - 1;
                let number_value: i32 = current_number_str.parse().unwrap_or(0);
                let mut is_part_number = false;

                let min_x = x.saturating_sub(1);
                let max_x = (x + 1).min(number_of_lines - 1);
                let min_y = start_y.saturating_sub(1);
                let max_y = (end_y + 1).min(total_chars - 1);

                for check_x in min_x..=max_x {
                    for check_y in min_y..=max_y {
                        let char_to_check = grid[check_x][check_y];

                        if !char_to_check.is_digit(10) && char_to_check != '.' {
                            is_part_number = true;
                            break;
                        }
                    }
                    if is_part_number {
                        break;
                    }
                }

                if is_part_number {
                    result += number_value;
                }

                y = current_y;

            } else {
                y += 1;
            }
        }
    }
    println!("Result : {}", result);
    result
}

pub fn solve_day3_part2(input: &str) -> i64 {

    // Découpage et préparation de la grille
    let lines: Vec<&str> = input.lines().collect();
    let number_of_lines = lines.len();
    let grid: Vec<Vec<char>> = lines.iter()
        .map(|line| line.chars().collect())
        .collect();
    let total_chars = grid.get(0).map_or(0, |row| row.len());

    // HashMap pour stocker les nombres adjacents à chaque '*'
    // Clé: (row, col) du '*' ; Valeur: Liste des nombres adjacents
    let mut gear_candidates: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    // **1. Trouver tous les nombres et leurs '*' adjacents**
    for x in 0..number_of_lines {
        let mut y = 0;

        while y < total_chars {
            let item = grid[x][y];

            if item.is_digit(10) {
                let mut current_number_str = String::new();
                let start_y = y;

                // A. Lire le nombre complet
                let mut current_y = y;
                while current_y < total_chars && grid[x][current_y].is_digit(10) {
                    current_number_str.push(grid[x][current_y]);
                    current_y += 1;
                }

                let end_y = current_y - 1;
                let number_value: i32 = current_number_str.parse().unwrap_or(0);

                // B. Vérifier la zone 3xN (Autour du nombre)
                let min_x = x.saturating_sub(1);
                let max_x = (x + 1).min(number_of_lines - 1);
                let min_y = start_y.saturating_sub(1);
                let max_y = (end_y + 1).min(total_chars - 1);

                for check_x in min_x..=max_x {
                    for check_y in min_y..=max_y {
                        let char_to_check = grid[check_x][check_y];

                        // Si on trouve un '*'
                        if char_to_check == '*' {
                            // Ajouter ce nombre à la liste des nombres adjacents à ce '*'
                            let gear_coord = (check_x, check_y);
                            gear_candidates.entry(gear_coord)
                                .or_insert_with(Vec::new)
                                .push(number_value);
                        }
                    }
                }

                // C. Avancer 'y' pour sauter les chiffres déjà traités
                y = current_y;

            } else {
                y += 1;
            }
        }
    }

    // **2. Calculer la somme des ratios**
    let mut total_gear_ratio: i64 = 0;

    for (_, adjacent_numbers) in gear_candidates.into_iter() {
        // Un engrenage (gear) doit être adjacent à EXACTEMENT deux nombres
        if adjacent_numbers.len() == 2 {
            // Calculer le ratio (produit des deux nombres)
            let ratio = adjacent_numbers[0] as i64 * adjacent_numbers[1] as i64;
            total_gear_ratio += ratio;
        }
    }

    println!("Result : {}", total_gear_ratio);
    total_gear_ratio
}