use std::collections::HashMap;

pub fn solve_day1_2023(input: &str) -> u32 {
    let lines = input.lines();
    let mut total: u32 = 0;

    for line in lines {
        let mut numbers: Vec<u32> = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c.to_digit(10).unwrap() as u32);
            }
        }
        total += count(&numbers);
    }
    println!("total : {}", total);
    let part_2 = solve_day1_2023_2(input);

    total
}

pub fn solve_day1_2023_2(input: &str) -> u32 {
    let list_numbers: HashMap<&str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
        .iter()
        .cloned()
        .collect();

    let mut total: u32 = 0;

    for line in input.lines() {
        let mut numbers: Vec<u32> = Vec::new();

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                let number = c.to_digit(10).unwrap() as u32;
                numbers.push(number);
            }else {
                for (key, value) in list_numbers.iter() {
                    let tmp_length = key.len();
                    if i + tmp_length <= line.len() {
                        let char_test = line[i..i + tmp_length].to_string();
                        if char_test == *key {
                            numbers.push(*value);
                        }
                    }
                }
            }
        }
        total += count(&numbers);
    }

    println!("total partie 2: {}", total);
    total
}

fn count(list: &Vec<u32>) -> u32 {
    if list.len() == 1 {
        format!("{}{}", list[0], list[0]).parse::<u32>().unwrap()
    }else if !list.is_empty() {
        format!("{}{}", list[0], list[list.len() - 1]).parse::<u32>().unwrap()
    } else {
        0
    }
}