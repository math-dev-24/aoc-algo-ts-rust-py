use regex::{Captures, Regex};
use std::collections::HashSet;

type Coordinate = (i32, i32);

#[derive(Debug)]
struct Robot {
    position: Coordinate,
    velocity: Coordinate,
}

impl Robot {
    fn new(s: &str) -> Self {
        let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let captures = regex.captures(s).expect("Invalid robot format");

        let position = (
            captures[1].parse::<i32>().unwrap(),
            captures[2].parse::<i32>().unwrap(),
        );
        let velocity = (
            captures[3].parse::<i32>().unwrap(),
            captures[4].parse::<i32>().unwrap(),
        );

        Self { position, velocity }
    }

    fn move_robot(&mut self) {
        const WIDTH: i32 = 101;
        const HEIGHT: i32 = 103;
        self.position.0 = ((self.position.0 + self.velocity.0) % WIDTH + WIDTH) % WIDTH;
        self.position.1 = ((self.position.1 + self.velocity.1) % HEIGHT + HEIGHT) % HEIGHT;
    }
}

pub fn solve_day14(input: &str) -> i32 {
    const WIDTH: i32 = 101;
    const MID_X: i32 = WIDTH / 2;
    const HEIGHT: i32 = 103;
    const MID_Y: i32 = HEIGHT / 2;

    // Parse the robots
    let mut robots: Vec<Robot> = input
        .trim()
        .lines()
        .map(|s| Robot::new(s))
        .collect();

    // Move all robots and calculate quadrants
    let mut quadrants = vec![0; 4];
    for robot in robots.iter_mut() {
        robot.move_robot();
        let (x, y) = robot.position;
        if x == MID_X || y == MID_Y {
            continue;
        }
        if x < MID_X && y < MID_Y {
            quadrants[0] += 1;
        } else if x >= MID_X && y < MID_Y {
            quadrants[1] += 1;
        } else if x < MID_X && y >= MID_Y {
            quadrants[2] += 1;
        } else if x >= MID_X && y >= MID_Y {
            quadrants[3] += 1;
        }
    }

    let safety_factor: i32 = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    println!("Résultat : {}", safety_factor);

    // Partie 2 : recherche du temps de stabilisation
    let mut actual_state: HashSet<String> = HashSet::new();
    let mut time_s = 0;

    while true {
        time_s += 1;

        // Avancer tous les robots
        for robot in robots.iter_mut() {
            robot.move_robot();
        }

        // Collecter les positions dans un HashSet
        let points: Vec<String> = robots
            .iter()
            .map(|robot| format!("{},{}", robot.position.0, robot.position.1))
            .collect();

        // Vérifier si les points sont uniques
        let unique_points: HashSet<_> = points.iter().collect();
        if unique_points.len() == points.len() {
            break;
        }

        // Vérifier si l'état s'est répété
        let state: String = points.join(",");
        if actual_state.contains(&state) {
            break;
        }
        actual_state.insert(state);
    }

    println!("Temps : {}", time_s);
    time_s
}