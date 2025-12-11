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
        // Solve using Gaussian elimination + DFS for free variables
        // This treats the problem as a system of linear equations:
        // Each button press increments certain counters by 1
        // We need to find minimum total presses to reach target values

        let num_buttons = self.buttons.len();
        let num_counters = self.target_joltages.len();

        if num_buttons == 0 || num_counters == 0 {
            return 0;
        }

        // Build augmented matrix [A | B] where:
        // A[i][j] = 1 if button j affects counter i, 0 otherwise
        // B[i] = target value for counter i
        let mut matrix = vec![vec![0.0; num_buttons + 1]; num_counters];

        for (button_idx, button_wiring) in self.buttons.iter().enumerate() {
            for &counter_idx in button_wiring {
                if counter_idx < num_counters {
                    matrix[counter_idx][button_idx] = 1.0;
                }
            }
        }

        for i in 0..num_counters {
            matrix[i][num_buttons] = self.target_joltages[i] as f64;
        }

        // Gaussian elimination to Row Echelon Form
        let mut pivot_row = 0;
        let mut col_to_pivot_row = vec![None; num_buttons];
        let mut free_cols = Vec::new();

        for col in 0..num_buttons {
            if pivot_row >= num_counters {
                free_cols.push(col);
                continue;
            }

            // Find best pivot (partial pivoting)
            let mut max_row = pivot_row;
            for row in pivot_row + 1..num_counters {
                if matrix[row][col].abs() > matrix[max_row][col].abs() {
                    max_row = row;
                }
            }

            // Skip if pivot is too small
            if matrix[max_row][col].abs() < 1e-9 {
                free_cols.push(col);
                continue;
            }

            matrix.swap(pivot_row, max_row);

            // Normalize pivot row
            let pivot_val = matrix[pivot_row][col];
            for c in col..=num_buttons {
                matrix[pivot_row][c] /= pivot_val;
            }

            // Eliminate other rows
            for row in 0..num_counters {
                if row != pivot_row {
                    let factor = matrix[row][col];
                    for c in col..=num_buttons {
                        matrix[row][c] -= factor * matrix[pivot_row][c];
                    }
                }
            }

            col_to_pivot_row[col] = Some(pivot_row);
            pivot_row += 1;
        }

        // Check for inconsistency
        for row in pivot_row..num_counters {
            if matrix[row][num_buttons].abs() > 1e-9 {
                return usize::MAX; // Impossible
            }
        }

        // If no free variables, compute directly
        if free_cols.is_empty() {
            let mut total = 0;
            for col in 0..num_buttons {
                if let Some(row) = col_to_pivot_row[col] {
                    let val = matrix[row][num_buttons];
                    if val < -1e-9 {
                        return usize::MAX;
                    }
                    let rounded = val.round();
                    if (val - rounded).abs() > 1e-9 {
                        return usize::MAX;
                    }
                    total += rounded as usize;
                }
            }
            return total;
        }

        // Compute weights for free variables
        let mut free_var_weights = Vec::new();
        for &col in &free_cols {
            let mut weight = 1.0;
            for row in 0..num_counters {
                weight -= matrix[row][col];
            }
            free_var_weights.push(weight);
        }

        // Base cost (assuming all free vars = 0)
        // This is the sum of all pivot variables when free vars = 0
        let mut base_cost = 0.0;
        for row in 0..num_counters {
            // Only count valid equation rows (not 0=0)
            if matrix[row][num_buttons].abs() > 1e-9
                || (0..num_buttons).any(|c| matrix[row][c].abs() > 1e-9)
            {
                base_cost += matrix[row][num_buttons];
            }
        }

        // DFS with pruning to find minimum
        let initial_rhs: Vec<f64> = (0..num_counters).map(|r| matrix[r][num_buttons]).collect();
        let mut min_presses = usize::MAX;
        let mut stack = vec![(0, vec![0i64; num_buttons], base_cost, initial_rhs)];

        while let Some((idx, solution, current_cost, current_rhs)) = stack.pop() {
            // Pruning: cost lower bound
            let mut min_future = 0.0;
            for i in idx..free_cols.len() {
                if free_var_weights[i] < 0.0 {
                    min_future += free_var_weights[i] * 300.0;
                }
            }
            if min_presses != usize::MAX && current_cost + min_future >= (min_presses as f64) - 1e-9 {
                continue;
            }

            // Pruning: feasibility check
            let mut feasible = true;
            for row in 0..num_counters {
                let mut min_future_sub = 0.0;
                for i in idx..free_cols.len() {
                    let col = free_cols[i];
                    let coeff = matrix[row][col];
                    if coeff < 0.0 {
                        min_future_sub += coeff * 300.0;
                    }
                }
                if current_rhs[row] < min_future_sub - 1e-9 {
                    feasible = false;
                    break;
                }
            }
            if !feasible {
                continue;
            }

            // All free variables assigned
            if idx == free_cols.len() {
                let mut valid = true;
                let mut total = 0i64;

                // Sum free vars
                for &col in &free_cols {
                    total += solution[col];
                }

                // Check and sum pivot vars
                for col in 0..num_buttons {
                    if let Some(row) = col_to_pivot_row[col] {
                        let val = current_rhs[row];
                        if val < -1e-9 {
                            valid = false;
                            break;
                        }
                        let rounded = val.round();
                        if (val - rounded).abs() > 1e-9 {
                            valid = false;
                            break;
                        }
                        total += rounded as i64;
                    }
                }

                if valid && total >= 0 && (total as usize) < min_presses {
                    min_presses = total as usize;
                }
                continue;
            }

            // Try values for next free variable
            let col = free_cols[idx];
            let weight = free_var_weights[idx];

            // Heuristic: If weight is positive, try small values first.
            // If weight is negative, try large values first.
            if weight >= 0.0 {
                for val in 0..=300 {
                    let mut new_sol = solution.clone();
                    new_sol[col] = val;

                    let mut new_rhs = current_rhs.clone();
                    for row in 0..num_counters {
                        new_rhs[row] -= matrix[row][col] * (val as f64);
                    }

                    let new_cost = current_cost + weight * (val as f64);
                    stack.push((idx + 1, new_sol, new_cost, new_rhs));
                }
            } else {
                for val in (0..=300).rev() {
                    let mut new_sol = solution.clone();
                    new_sol[col] = val;

                    let mut new_rhs = current_rhs.clone();
                    for row in 0..num_counters {
                        new_rhs[row] -= matrix[row][col] * (val as f64);
                    }

                    let new_cost = current_cost + weight * (val as f64);
                    stack.push((idx + 1, new_sol, new_cost, new_rhs));
                }
            }
        }

        if min_presses == usize::MAX { 0 } else { min_presses }
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
