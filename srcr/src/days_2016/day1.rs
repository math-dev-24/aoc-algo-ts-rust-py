use std::collections::HashSet;

pub fn solve_day_1(input: &str) -> u32 {
    let mut x = 0_i32;
    let mut y = 0_i32;
    let mut direction = 0;
    let mut visited_positions = HashSet::new();

    let calc = |x: i32, y: i32| -> u32 {
        (x.abs() + y.abs()) as u32
    };

    visited_positions.insert((x, y));

    let binding = input.clone().replace("\n", "");

    let instructions = binding.split(", ").collect::<Vec<&str>>();

    for instruction in instructions {
        let (turn, step_txt) = instruction.split_at(1);
        let steps = match step_txt.parse::<i32>() {
            Ok(x) => x,
            Err(_) => panic!("Invalid step txt {}", step_txt),
        };

        match turn {
            "R" => direction = (direction + 1) % 4,
            "L" => direction = (direction + 3) % 4,
            _ => panic!("Invalid turn {}", turn),
        }

        for _ in 0..steps {
            match direction {
                0 => y += 1,   // North
                1 => x += 1,   // East
                2 => y -= 1,   // South
                3 => x -= 1,   // West
                _ => unreachable!(),
            }

            if !visited_positions.insert((x, y)) {
                println!("First visited twice at position: ({}, {})", x, y);
                println!("Result: {}", calc(x, y));
                return calc(x, y);
            }
        }
    }
    let response = calc(x, y);
    println!("Résultat part 1 : {}", response);

    response
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(solve_day_1("R2, L3"), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(solve_day_1("R2, R2, R2"), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(solve_day_1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_example_4() {
        assert_eq!(solve_day_1("R8, R4, R4, R8"), 4);
    }
}
