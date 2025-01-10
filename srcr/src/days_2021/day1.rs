use std::collections::HashMap;

pub fn solve_day1(input: &str) -> u32 {
    let numbers: Vec<usize> = input
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    // part 1
    let result_1 = part_1(&numbers);

    // part 2
    let letters_count = (numbers.len() + 2).min(26);
    let mut part_2 = vec![0; letters_count];

    for (i, &line) in numbers.iter().enumerate() {
        if i >= 2 {
            part_2[(i - 2) % letters_count] += line;
        }
        if i >= 1 {
            part_2[(i - 1) % letters_count] += line;
        }
        part_2[i % letters_count] += line;
    }

    let result_2 = part_1(&part_2);

    println!("result part 1: {}", result_1);
    println!("result part 2: {}", result_2);
    0
}

fn part_1(data: &[usize]) -> usize {
    data.windows(2).filter(|w| w[1] > w[0]).count()
}