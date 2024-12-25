pub fn solve_day_7(input: &str) {
    let operators: Vec<char> = vec!['+', '*', '|'];
    let mut total = 0;

    for line in input.lines() {
        let target = line.trim().split(':').next().unwrap().trim().parse::<u128>().unwrap();
        let numbers = line.split(':')
            .last()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();

        let valid_operations = find_valid_operations(&numbers, &target, &operators);
        if valid_operations > 0 {
            total += target;
        }
    }
    println!("total : {}", total);
}

fn find_valid_operations(numbers: &Vec<u128>, target: &u128, operators: &Vec<char>) -> u128 {
    let operators_combinations = generate_operations_combinations(numbers.len() - 1, operators);

    let mut valid_count: u128 = 0;

    for ops in operators_combinations {
        let mut result = numbers[0];

        for (i, op) in ops.iter().enumerate() {
            let next_number = numbers[i + 1];
            match op {
                '+' => result += next_number,
                '*' => result *= next_number,
                '|' => {
                    let concatenated = format!("{}{}", result, next_number).parse::<u128>().unwrap();
                    result = concatenated;
                }
                _ => panic!("Invalid operator"),
            }
        }
        if result == *target {
            valid_count += 1;
        }
    }
    valid_count
}

fn generate_operations_combinations(count: usize, operators: &Vec<char>) -> Vec<Vec<char>> {
    if count == 0 {
        return vec![vec![]];
    }
    let smaller_combos = generate_operations_combinations(count - 1, operators);

    smaller_combos
        .into_iter()
        .flat_map(|combo| {
            operators.iter().map(move |&op| {
                let mut new_combo = combo.clone();
                new_combo.push(op);
                new_combo
            })
        })
        .collect()
}