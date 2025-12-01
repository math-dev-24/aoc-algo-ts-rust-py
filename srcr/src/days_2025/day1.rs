pub fn solve_day_1(input: &str) -> (usize, usize) {
    let part1 = part_1(input);
    let part2 = part_2(input);

    (part1, part2)
}

fn part_2(input: &str) -> usize {
    let mut pointer: isize = 50;
    let mut count: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let rest: String = chars.collect();
        let angle = rest.parse::<isize>().unwrap();

        let start = pointer;
        let end = if dir == 'L' {
            pointer - angle
        } else {
            pointer + angle
        };

        // Compter les passages par 0
        count += count_zeros(start, end);

        pointer = end.rem_euclid(100);
    }

    println!("Résultat partie 2 : {}", count);
    count
}

fn count_zeros(start: isize, end: isize) -> usize {
    if end > start {
        let first_cross = (start / 100 + 1) * 100;
        if first_cross > end {
            0
        } else {
            ((end - first_cross) / 100 + 1) as usize
        }
    } else if end < start {
        let first_cross = if start % 100 == 0 {
            start - 100
        } else {
            (start / 100) * 100
        };
        if first_cross < end {
            0
        } else {
            ((first_cross - end) / 100 + 1) as usize
        }
    } else {
        0
    }
}

fn part_1(input: &str) -> usize {
    let mut pointer: isize = 50;
    let mut count: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        let rest: String = chars.collect();
        let angle = rest.parse::<isize>().unwrap();

        if dir == 'L' {
            pointer -= angle;
        } else {
            pointer += angle;
        }

        pointer = pointer.rem_euclid(100);

        if pointer == 0 {
            count += 1;
        }
    }

    println!("Résultat partie 1 : {}", count);

    count
}
