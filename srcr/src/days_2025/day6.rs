use std::collections::HashMap;



pub fn solve_day_6(input: &str) {

    let mut problems: HashMap<usize, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut col_index = 0;

        for item in line.split_whitespace() {
            problems.entry(col_index)
                .or_insert(Vec::new())
                .push(item.to_string());
            col_index += 1;
        }

    }

    let mut resultats = HashMap::new();
    for (col, values) in problems.iter() {
    if let Some(op) = values.last() {
        if op == "*" || op == "+" {
            let nombres: Vec<i64> = values[..values.len()-1]
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect();
            
            let resultat = match op.as_str() {
                "*" => nombres.iter().product(),
                "+" => nombres.iter().sum(),
                _ => 0
            };
            
            resultats.insert(*col, resultat);
        }
    }

    }

    let somme_totale: i64 = resultats.values().sum();


    println!("{:?}", somme_totale); 
}

pub fn solve_day_6_part2(input: &str) {


    let lines: Vec<&str> = input.lines().collect();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    let mut resultats = Vec::new();
    let mut current_problem: Vec<i64> = Vec::new();
    let mut current_op = "";
    
    // Parcourir de droite à gauche
    for col in (0..max_len).rev() {
        let mut vertical_num = String::new();
        let mut is_operator = false;
        
        // Lire verticalement chaque colonne
        for line in &lines {
            if let Some(ch) = line.chars().nth(col) {
                if ch == '*' || ch == '+' {
                    is_operator = true;
                    current_op = if ch == '*' { "*" } else { "+" };
                } else if ch.is_digit(10) {
                    vertical_num.push(ch);
                }
            }
        }
        
        // Si on a un nombre, l'ajouter au problème courant
        if !vertical_num.is_empty() {
            if let Ok(num) = vertical_num.parse::<i64>() {
                current_problem.push(num);
            }
        }
        
        // Si on atteint un opérateur ou un espace vide, calculer
        if is_operator || (col > 0 && vertical_num.is_empty() && !current_problem.is_empty()) {
            if !current_problem.is_empty() && !current_op.is_empty() {
                let resultat = match current_op {
                    "*" => current_problem.iter().product(),
                    "+" => current_problem.iter().sum(),
                    _ => 0
                };
                resultats.push(resultat);
                current_problem.clear();
            }
        }
    }
    
    let somme_totale: i64 = resultats.iter().sum();
    println!("Résultats: {:?}", resultats);
    println!("Somme totale: {}", somme_totale);
}