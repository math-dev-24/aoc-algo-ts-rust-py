use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Machine {
    target_lights: Vec<bool>,
    target_joltages: Vec<usize>,
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split_whitespace().collect();

        // Parse target state from [.##.]
        let target_str = parts[0].trim_matches(|c| c == '[' || c == ']');
        let target_lights: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

        // Parse buttons from (1,3) format
        let mut buttons = Vec::new();
        let mut joltage_str = "";

        for part in &parts[1..] {
            if part.starts_with('(') && part.ends_with(')') {
                let button_str = part.trim_matches(|c| c == '(' || c == ')');
                if !button_str.is_empty() {
                    let indices: Vec<usize> =
                        button_str.split(',').map(|s| s.parse().unwrap()).collect();
                    buttons.push(indices);
                }
            } else if part.starts_with('{') {
                joltage_str = part;
            }
        }

        // Parse joltage requirements from {3,5,4,7}
        let target_joltages: Vec<usize> = if !joltage_str.is_empty() {
            joltage_str
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        } else {
            vec![]
        };

        Machine {
            target_lights,
            target_joltages,
            buttons,
        }
    }

    fn min_presses_lights(&self) -> usize {
        let n_lights = self.target_lights.len();

        // BFS to find minimum button presses
        let mut queue = VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        let initial_state = vec![false; n_lights];
        queue.push_back((initial_state.clone(), 0));
        visited.insert(initial_state);

        while let Some((state, presses)) = queue.pop_front() {
            if state == self.target_lights {
                return presses;
            }

            for button in &self.buttons {
                let mut new_state = state.clone();
                for &light_idx in button {
                    if light_idx < n_lights {
                        new_state[light_idx] = !new_state[light_idx];
                    }
                }

                if !visited.contains(&new_state) {
                    visited.insert(new_state.clone());
                    queue.push_back((new_state, presses + 1));
                }
            }
        }

        0
    }

    fn min_presses_joltages(&self) -> usize {
        // BFS avec pruning intelligent basé sur la taille du problème
        use std::collections::{HashMap, VecDeque};

        let n_counters = self.target_joltages.len();
        let total_target: usize = self.target_joltages.iter().sum();

        // Pour les petits problèmes, BFS complet
        if total_target <= 60 {
            let mut queue = VecDeque::new();
            let mut visited = HashMap::new();

            queue.push_back((vec![0; n_counters], 0));
            visited.insert(vec![0; n_counters], 0);

            while let Some((state, presses)) = queue.pop_front() {
                if state == self.target_joltages {
                    return presses;
                }

                for button in &self.buttons {
                    let mut new_state = state.clone();
                    let mut valid = true;

                    for &idx in button {
                        if idx < n_counters {
                            new_state[idx] += 1;
                            if new_state[idx] > self.target_joltages[idx] {
                                valid = false;
                                break;
                            }
                        }
                    }

                    if valid {
                        if !visited.contains_key(&new_state) {
                            visited.insert(new_state.clone(), presses + 1);
                            queue.push_back((new_state, presses + 1));
                        }
                    }
                }
            }

            return *visited.get(&self.target_joltages).unwrap_or(&usize::MAX);
        }

        // Pour les gros problèmes, BFS avec limite de mémoire
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back((vec![0; n_counters], 0));
        visited.insert(vec![0; n_counters], 0);

        while let Some((state, presses)) = queue.pop_front() {
            if state == self.target_joltages {
                return presses;
            }

            // Pruning: si trop d'états, nettoyer les anciens
            if visited.len() > 200_000 {
                visited.retain(|_, &mut v| v >= presses.saturating_sub(5));
            }

            for button in &self.buttons {
                let mut new_state = state.clone();
                let mut valid = true;

                for &idx in button {
                    if idx < n_counters {
                        new_state[idx] += 1;
                        if new_state[idx] > self.target_joltages[idx] {
                            valid = false;
                            break;
                        }
                    }
                }

                if valid {
                    let new_presses = presses + 1;
                    if !visited.contains_key(&new_state) || visited[&new_state] > new_presses {
                        visited.insert(new_state.clone(), new_presses);
                        queue.push_back((new_state, new_presses));
                    }
                }
            }

            // Limite de sécurité
            if presses > total_target * 2 {
                break;
            }
        }

        *visited.get(&self.target_joltages).unwrap_or(&usize::MAX)
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .par_lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = Machine::parse(line);
            machine.min_presses_lights()
        })
        .reduce(|| 0, |a, b| a.saturating_add(b))
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = Machine::parse(line);
            let result = machine.min_presses_joltages();
            if result == usize::MAX {
                eprintln!("Warning: No solution found for: {}", line);
                0
            } else {
                result
            }
        })
        .sum()
}

pub fn solve_day_10(input: &str) -> usize {
    let part1 = solve_part_1(input);
    println!("Partie 1 : {}", part1);
    let part2 = solve_part_2(input);
    println!("Partie 2 : {}", part2);

    part1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        assert_eq!(solve_part_1(input), 7);
    }

    #[test]
    fn test_part2_example() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        assert_eq!(solve_part_2(input), 33);
    }

    #[test]
    fn test_first_machine_lights() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = Machine::parse(input);
        assert_eq!(machine.min_presses_lights(), 2);
    }

    #[test]
    fn test_first_machine_joltages() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = Machine::parse(input);
        assert_eq!(machine.min_presses_joltages(), 10);
    }

    #[test]
    fn test_second_machine_joltages() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = Machine::parse(input);
        assert_eq!(machine.min_presses_joltages(), 12);
    }

    #[test]
    fn test_third_machine_joltages() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = Machine::parse(input);
        assert_eq!(machine.min_presses_joltages(), 11);
    }
}
