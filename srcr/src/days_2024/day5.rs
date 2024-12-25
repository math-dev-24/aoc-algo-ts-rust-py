use std::collections::HashMap;
use std::fs;
use std::time::Instant;

pub fn solve_day_5() {
    let input = fs::read_to_string("assets/day5.txt").expect("Fichier introuvable");
    let start = Instant::now();
    let (rules, updates) = parse_input(&input);

    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut unordered_updates = Vec::new();

    for update in &updates {
        if is_update_ordered(update, &rules) {
            total_part_1 += get_middle(update);
        } else {
            unordered_updates.push(update.clone());
        }
    }
    
    let graph: HashMap<i32, Vec<i32>> = build_graph(&rules);

    for bad in &unordered_updates {
        let tempo = topological_sort(&graph, bad);
        total_part_2 += get_middle(&tempo)

    }


    println!("Partie 1 : {}", total_part_1);
    println!("Partie 2 : {}", total_part_2);
    println!("Temps total : {:.2?}", start.elapsed());
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    // Section des règles
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<i32> = line.split('|').map(|x| x.trim().parse().unwrap()).collect();
        rules.push((parts[0], parts[1]));
    }

    // Section des mises à jour
    for line in lines {
        let update: Vec<i32> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
        updates.push(update);
    }

    (rules, updates)
}


fn is_update_ordered(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for &(a, b) in rules {
        if let (Some(pos_a), Some(pos_b)) = (
            update.iter().position(|&x| x == a),
            update.iter().position(|&x| x == b),
        ) {
            if pos_a > pos_b {
                return false;
            }
        }
    }
    true
}


fn get_middle(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}


fn build_graph(rules: &Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    for &(a, b) in rules {
        graph.entry(a).or_insert(Vec::new()).push(b);
    }
    graph
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    for &node in update {
        in_degree.insert(node, 0);
    }


    for (&node, neighbors) in graph.iter() {
        for &neighbor in neighbors {
            if update.contains(&neighbor) {
                *in_degree.entry(neighbor).or_insert(0) += 1;
            }
        }
    }


    let mut queue: Vec<i32> = update
        .iter()
        .cloned()
        .filter(|&node| *in_degree.get(&node).unwrap_or(&0) == 0)
        .collect();

    if queue.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    while let Some(current) = queue.pop() {
        result.push(current);
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if update.contains(&neighbor) {
                    let entry = in_degree.entry(neighbor).or_insert(0);
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push(neighbor);
                    }
                }
            }
        }
    }

    println!("Sorted result: {:?}", result); // Vérifiez le résultat final
    result
}
