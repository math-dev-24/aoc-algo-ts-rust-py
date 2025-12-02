

pub fn solve_day_2_part_1(input: &str) -> usize {

    let list_of_range = input.split(",")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let mut invalid_part_1: usize = 0;
    let mut invalid_part_2: usize = 0;

    for tmp_range in list_of_range {
        let start_end = tmp_range.split("-").collect::<Vec<&str>>();

        let start = start_end[0].parse::<usize>().unwrap();
        let end = start_end[1].parse::<usize>().unwrap();

        if start > end {
            panic!("Error: start ne peux pas être supérieur à end");
        }

        for value in start..=end {
            if is_invalid(value) {
                invalid_part_1 += value;
            }
            if is_invalid_part_2(value) {
                invalid_part_2 += value;
            }
        }
    }


    println!("Résultat Part 1 : {}", invalid_part_1);
    println!("Résultat Part 2 : {}", invalid_part_2);

    invalid_part_1
}

fn is_invalid(number: usize) -> bool {
    let number_str = number.to_string();
    let len = number_str.len();
    
    if len % 2 != 0 {
        return false;
    }
    
    let mid = len / 2;
    let first_half = &number_str[..mid];
    let second_half = &number_str[mid..];
    
    first_half == second_half
}


fn is_invalid_part_2(number: usize) -> bool {
    let s = number.to_string();

    let len = s.len();
    
    if len < 2 {
        return false;
    }

    for pattern_len in 1..=(len / 2) {
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let mut is_valid_pattern = true;
        
        for i in (pattern_len..len).step_by(pattern_len) {
            if &s[i..i + pattern_len] != pattern {
                is_valid_pattern = false;
                break;
            }
        }
        
        if is_valid_pattern {
            return true;
        }
    }
    
    false
}