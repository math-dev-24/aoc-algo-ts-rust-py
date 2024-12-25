use regex::Regex;

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Prize {
    x: usize,
    y: usize,
}

impl Button {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let captures = re.captures(input).expect("Invalid button format");
        let x = captures[1].parse::<usize>().unwrap();
        let y = captures[2].parse::<usize>().unwrap();
        Self { x, y }
    }
}

impl Prize {
    fn new(input: &str, offset: usize) -> Self {
        let re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        let captures = re.captures(input).expect("Invalid prize format");
        let x = captures[1].parse::<usize>().unwrap() + offset;
        let y = captures[2].parse::<usize>().unwrap() + offset;
        Self { x, y }
    }
}

pub fn solve_day13(input: &str) -> u32 {
    let list: Vec<&str> = input.trim().split("\n").collect();
    let mut result = 0;
    let mut result_2 = 0;

    for i in (0..list.len()).step_by(4) {
        let button1 = Button::new(&list[i]);
        let button2 = Button::new(&list[i + 1]);
        let prize = Prize::new(&list[i + 2], 0);
        let prize_2 = Prize::new(&list[i + 2], 10000000000000);
        result += method_substitute(&button1, &button2, prize);
        result_2 += method_substitute(&button1, &button2, prize_2);
    }

    println!("Partie 1 : {}", result);
    println!("Partie 2 : {}", result_2);
    result as u32
}

fn method_substitute(button1: &Button, button2: &Button, prize: Prize) -> i128 {
    let Ax = button1.x as i128;
    let Ay = button1.y as i128;
    let Bx = button2.x as i128;
    let By = button2.y as i128;
    let Px = prize.x as i128;
    let Py = prize.y as i128;

    let denominator = Ay * Bx - Ax * By;
    if denominator == 0 {
        return -1;
    }

    let b: i128 = (Ay * Px - Ax * Py) / denominator;
    if Ay == 0 {
        return -1;
    }
    let a: i128 = (Py - By * b) / Ay;

    let xOk = (Ax * a + Bx * b) == Px;
    let yOk = (Ay * a + By * b) == Py;

    if !xOk || !yOk {
        return -1;
    }
    (3 * a + b) as i128
}