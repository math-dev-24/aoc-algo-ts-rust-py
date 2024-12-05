use std::collections::HashMap;
use std::fs;
use std::time::Instant;

pub fn solve_day_5() {
    let input = fs::read_to_string("assets/day5.txt").expect("Fichier introuvable");
    let start = Instant::now();

    let (rules, updates) = parse_input(&input);

    let graph = build_graph(&rules);

    let mut sum_of_middles = 0;

    for update in updates {
        let sorted_update = topological_sort(&graph, &update);
        if !sorted_update.is_empty() {
            let middle = sorted_update[sorted_update.len() / 2];
            sum_of_middles += middle;
        }
    }

    println!("Somme des milieux : {}", sum_of_middles);
    println!("Temps total : {:.2?}", start.elapsed());
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<i32> = line
            .split('|')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        rules.push((parts[0], parts[1]));
    }

    for line in lines {
        let update: Vec<i32> = line
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();
        updates.push(update);
    }

    (rules, updates)
}

fn build_graph(rules: &Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(rules.len());
    for &(a, b) in rules {
        graph.entry(a).or_insert(Vec::new()).push(b);
    }
    graph
}

fn topological_sort(graph: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
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

    let mut queue: Vec<i32> = Vec::new();
    for &node in update {
        if *in_degree.get(&node).unwrap_or(&0) == 0 {
            queue.push(node);
        }
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

    result
}
