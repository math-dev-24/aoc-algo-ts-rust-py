pub fn solve_day_3(input: &str) -> u64 {
    const TARGET: usize = 12;

    let total = input.lines()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let n = chars.len();

            let mut selected = Vec::with_capacity(TARGET);
            let mut start = 0;

            for pos in 0..TARGET {
                let window_end = n - (TARGET - pos - 1);

                let mut best_digit = '0';
                let mut best_idx = start;

                for i in start..window_end {
                    if chars[i] > best_digit {
                        best_digit = chars[i];
                        best_idx = i;
                    }
                }

                selected.push(best_digit);
                start = best_idx + 1;
            }

            let joltage = selected.iter()
                .fold(0u64, |acc, &ch| acc * 10 + ch.to_digit(10).unwrap() as u64);

            Some(joltage)
        })
        .sum();

    println!("Total : {}", total);
    total
}