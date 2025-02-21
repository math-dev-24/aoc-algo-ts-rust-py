use std::collections::VecDeque;

pub fn solve_day_5(input: &str) -> usize {
    let data: Vec<&str> = input.split("\n\n").collect();

    let stacks_input = data[0].lines().map(|e| e.to_string()).collect::<Vec<String>>();
    let instructions = data[1].lines().map(|e| e.to_string()).collect::<Vec<String>>();


    let mut stacks: Vec<VecDeque<char>> = vec![];

    for line in stacks_input.iter().rev().skip(1) {
        let char: Vec<char> = line.chars().collect();

        for (i, chunk) in char.chunks(4).enumerate() {
            if stacks.len() <= i {
                stacks.push(VecDeque::new());
            }
            if let Some(&c) = chunk.get(1) {
                if c.is_alphabetic() {
                    stacks[i].push_back(c);
                }
            }
        }
    }

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let count: usize = parts[1].parse().unwrap();
        let from: usize = parts[3].parse::<usize>().unwrap() - 1;
        let to: usize = parts[5].parse::<usize>().unwrap() - 1;

        // Extraire les caisses en conservant l'ordre
        let mut temp_stack = Vec::new();
        for _ in 0..count {
            if let Some(crate_char) = stacks[from].pop_back() {
                temp_stack.push(crate_char);
            }
        }
        temp_stack.reverse(); // On garde l'ordre original

        // Empiler les caisses dans la pile destination
        for crate_char in temp_stack {
            stacks[to].push_back(crate_char);
        }
    }


    // Lire le haut de chaque pile
    let result = stacks.iter()
        .map(|stack| stack.back().unwrap_or(&' ').to_string())
        .collect::<String>();

    println!("{}", result);
    1
}