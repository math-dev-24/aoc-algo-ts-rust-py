use std::collections::HashSet;
use itertools::Itertools;

pub fn solve_day_3(input: &str) -> usize {
    let rucksacks = input.lines();


    // partie 1
    let mut letters: Vec<char> = Vec::new();
    for rucksack in rucksacks.clone() {
        let middle = rucksack.len() / 2;
        let bag = rucksack.chars().collect::<Vec<char>>();
        let left = &bag[0..middle];
        let right = &bag[middle..rucksack.len()];
        for char in left{
            if right.contains(char) {
                letters.push(char.clone());
                break
            }
        }
    }
    let mut total_part_1: usize = 0;
    for letter in letters {
        total_part_1 += get_priority(&letter);
    }
    println!("Partie 1 : {}", total_part_1);

    // partie 2
    let mut total_part_2: usize = 0;

    for win in &rucksacks.chunks(3) {
        let bags = win.collect::<Vec<&str>>();
        let bag1: HashSet<char> = bags[0].chars().collect();
        let bag2: HashSet<char> = bags[1].chars().collect();
        let bag3: HashSet<char> = bags[2].chars().collect();

        let badge = bag1.intersection(&bag2)
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&bag3)
            .cloned()
            .collect::<Vec<char>>();

        if let Some(char) = badge.first() {
            total_part_2 += get_priority(char);
        }
    }
    println!("Partie 2: {}", total_part_2);

    1
}



fn get_priority(char: &char) -> usize {
    if char.is_ascii_lowercase() {
        *char as usize - 'a' as usize + 1
    } else if char.is_ascii_uppercase() {
        *char as usize - 'A' as usize + 27
    } else {
        0
    }
}