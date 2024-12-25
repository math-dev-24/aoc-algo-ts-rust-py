use std::collections::{HashMap, HashSet};

pub fn solve_day_23(input: &str) -> usize {
    let data: Vec<&str> = input.trim().split("\n").collect::<Vec<&str>>();
    let mut pairs: Vec<(&str, &str)> = Vec::new();
    let mut network: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut listings: HashSet<(String, String, String)> = HashSet::new();

    for line in data {
        let mut split = line.split("-");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        pairs.push((first, second));
    }

    for pair in pairs {
        let first = pair.0;
        let second = pair.1;
        network.entry(first).or_insert(Vec::new()).push(second);
        network.entry(second).or_insert(Vec::new()).push(first);
    }

    for (&a, neighbors_a) in &network {
        for &b in neighbors_a {
            if let Some(neighbors_b) = network.get(b) {
                for &c in neighbors_b {
                    if let Some(neighbors_c) = network.get(c) {
                        if neighbors_c.contains(&a) && a != b && b != c && a != c {
                            let mut triplet = vec![a.to_string(), b.to_string(), c.to_string()];
                            triplet.sort();
                            listings.insert((triplet[0].clone(), triplet[1].clone(), triplet[2].clone()));
                        }
                    }
                }
            }
        }
    }

    let filtered_listings: Vec<&(String, String, String)> = listings
        .iter()
        .filter(|triplet| {
            triplet.0.starts_with('t') || triplet.1.starts_with('t') || triplet.2.starts_with('t')
        })
        .collect();

    println!("{:?}", filtered_listings.len());

    filtered_listings.len()
}