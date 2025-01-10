
struct Point {
    horizontal: usize,
    vertical: usize,
    vise: usize,
}

impl Point {
    fn action_part_1(&mut self, action: &str, value: usize) {
        match action {
            "forward" => self.inc_horizontal(value),
            "down" => self.inc_vertical(value),
            "up" => self.dec_vertical(value),
            _ => {}
        }
    }
    fn action_part_2(&mut self, action: &str, value: usize) {
        match action {
            "down" => self.inc_vise(value),
            "up" => self.dec_vise(value),
            "forward" => {
                self.inc_horizontal(value);
                self.inc_vertical(self.vise * value);
            },
            _ => {}
        }
    }
    fn inc_horizontal(&mut self, value: usize) {
        self.horizontal += value;
    }
    fn inc_vertical(&mut self, value: usize) {
        self.vertical += value;
    }
    fn dec_vertical(&mut self, value: usize) {
        self.vertical -= value;
    }
    fn inc_vise(&mut self, value: usize) {
        self.vise += value;
    }
    fn dec_vise(&mut self, value: usize) {
        self.vise -= value;
    }
    fn get_score(&self) -> usize {
        self.horizontal * self.vertical
    }
}


pub fn solve_day2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut point: Point = Point {
        horizontal: 0,
        vertical: 0,
        vise: 0,
    };
    let mut point_2: Point = Point {
        horizontal: 0,
        vertical: 0,
        vise: 0,
    };

    for line in lines {
        let input: Vec<&str> = line.split(" ").collect();
        let action = input[0];
        let value = input[1].parse::<usize>().unwrap();
        point.action_part_1(action, value);
        point_2.action_part_2(action, value);
    }
    println!("Partie 1 : {}", point.get_score());
    println!("Partie 2 : {}", point_2.get_score());
    0
}