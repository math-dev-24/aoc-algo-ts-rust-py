pub fn solve_day_25(input: &str) -> u32 {
    let schematics: Vec<&str> = input.lines().collect();
    let items = schematics.len();
    let mut keys: Vec<Vec<usize>> = Vec::new();
    let mut locks: Vec<Vec<usize>> = Vec::new();
    let mut index = 0;

    while index + 7 <= items {
        let tmp_schematic = &schematics[index..index + 7];

        if tmp_schematic[0] == "....." {
            let key: Vec<usize> = parse_schematic(tmp_schematic.to_vec(), false, 5);
            keys.push(key);
        } else {
            let lock: Vec<usize> = parse_schematic(tmp_schematic.to_vec(), true, 5);
            locks.push(lock);
        }
        index += 8;
    }
    let valid_pairs = count_valid_pairs(&locks, &keys, 5);
    println!("valid_pairs: {}", valid_pairs);

    valid_pairs as u32
}

fn parse_schematic(schema: Vec<&str>, is_lock: bool, total_height: usize) -> Vec<usize> {
    let rows: Vec<String> = schema.iter().map(|line| line.trim().to_string()).collect();

    if is_lock {
        let rows_locked = &rows[1..];
        (0..rows_locked[0].len())
            .map(|col_idx| {
                rows_locked
                    .iter()
                    .map(|row| row.chars().nth(col_idx).unwrap())
                    .position(|c| c == '.')
                    .unwrap_or(total_height)
            })
            .collect()
    } else {
        let reversed_rows: Vec<String> = rows.into_iter().rev().collect();
        let rows_key = &reversed_rows[1..];
        (0..rows_key[0].len())
            .map(|col_idx| {
                rows_key
                    .iter()
                    .map(|row| row.chars().nth(col_idx).unwrap())
                    .position(|c| c == '.')
                    .unwrap_or(total_height)
            })
            .collect()
    }
}

fn count_valid_pairs(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>, total_height: usize) -> usize {
    let mut valid_pairs = 0;

    for lock in locks {
        for key in keys {
            if lock.iter()
                .zip(key.iter())
                .all(|(l, k)| l + k <= total_height)
            {
                valid_pairs += 1;
            }
        }
    }
    valid_pairs
}