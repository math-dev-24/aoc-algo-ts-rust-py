use std::fs;

struct State {
    is_valid: bool,
    q_error: u8
}

fn is_valid(array: &Vec<u128>) -> State {
    let d: i8 = if array[0] < array[1] { 1 } else { -1 };
    let mut q_error: u8 = 0;

    for i in 0..(array.len() - 1){
        let value: u128 = array[i];
        let next_value: u128 = array[i + 1];
        let tmp_d: i8 = if value < next_value { 1 } else { -1 };
        let delta: i128 = value as i128 - next_value as i128;

        if d != tmp_d || delta == 0 || delta.abs() > 3{
            q_error += 1;
            if q_error > 1 { 
                return State {is_valid: false, q_error};
            }
        }
    }
    State {
        is_valid: q_error == 0,
        q_error
    }

}

pub fn solve_day_2() {
    let input: String = fs::read_to_string("assets/day2.txt").expect("Fichier introuvable");
    let mut counter: u32 = 0;

    for line in input.trim().split("\n"){
        let tmp_line: Vec<u128> = line
        .trim()
        .split(' ')
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

        let state = is_valid(&tmp_line);
        if state.is_valid || state.q_error == 1 {
            counter+= 1;
        }
    }
    println!("Result : {}", counter);
}