use std::collections::HashMap;
use std::fs;

pub fn solve_day_1() {
    let input: String = fs::read_to_string("assets/day1.txt").expect("Fichier introuvable");

    let mut deck_1: Vec<u32> = Vec::new();
    let mut deck_2: Vec<u32> = Vec::new();

    for line in input.trim().lines() {
        let values: Vec<&str> = line.split_whitespace().collect();

        if let Some(value) = values.get(0) {
            match value.parse::<u32>() {
                Ok(num) => deck_1.push(num),
                Err(_) => eprintln!("Erreur lors du parsing de la valeur : {}", value),
            }
        }
        if let Some(value) = values.get(1) {
            match value.parse::<u32>() {
                Ok(num) => deck_2.push(num),
                Err(_) => eprintln!("Erreur lors du parsing de la valeur : {}", value),
            }
        }
    }
    deck_1.sort();
    deck_2.sort();
    let result_1: u32 = part_1(&deck_1, &deck_2);
    let result_2: u32 = part_2(&deck_1, &deck_2);

    println!("Result partie 1 : {}", result_1);
    println!("Result partie 2 : {}", result_2);


}

fn part_2(deck_1: &Vec<u32>, deck_2: &Vec<u32>) -> u32 {
    let mut counts = HashMap::new();

    for &num in deck_2 {
        *counts.entry(num).or_insert(0) += 1;
    }

    let total: u32 = deck_1.iter()
        .map(|&num| {
            let count = counts.get(&num).unwrap_or(&0);
            num * count
        })
        .sum();

    total
}

fn part_1(deck_1: &Vec<u32>, deck_2: &Vec<u32>) -> u32 {
    deck_1.iter()
        .zip(deck_2.iter())
        .map(|(num1, num2)| (num1 as &u32).abs_diff(*num2))
        .sum()
}
