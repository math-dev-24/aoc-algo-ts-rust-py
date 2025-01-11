
pub fn solve_day3(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let length: usize = lines[0].len();

    let mut gamma: Vec<usize> = vec![0; length];
    let mut epsilon: Vec<usize> = vec![0; length];

    for l in 0..length {
        let mut one_count: usize = 0;
        let mut zero_count: usize = 0;

        for line in lines.iter() {
            let tmp_val = line.chars().nth(l).unwrap();
            if tmp_val == '1' {
                one_count += 1;
            }
            if tmp_val == '0' {
                zero_count += 1;
            }
        }

        if one_count > zero_count {
            gamma[l] = 1;
            epsilon[l] = 0;
        }else{
            gamma[l] = 0;
            epsilon[l] = 1;
        }
    }

    let result_gamma = gamma.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("");

    let mut total_part_1 = 0;


    match i32::from_str_radix(&result_gamma, 2) {
        Ok(x) => {
            println!("result gamma: {}", x);
            total_part_1 = x;
        },
        Err(e) => println!("Erreur gamma: {}", e),
    }

    let result_epsilon = epsilon.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("");

    match i32::from_str_radix(&result_epsilon, 2) {
        Ok(x) => {
            println!("result epsilon: {}", x);
            total_part_1 = total_part_1 * x;
        },
        Err(e) => println!("Erreur epsilon: {}", e),
    }

    println!("result part 1: {}", total_part_1);

    0
}