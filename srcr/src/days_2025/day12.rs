
pub fn solve_day_12_2025(input: &str) -> usize {
    let puzzle: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let sizes: Vec<usize> = (1..24)
        .step_by(4)
        .filter_map(|i| {
            if i + 2 < puzzle.len() {
                Some(
                    puzzle[i..=i + 2]
                        .iter()
                        .flat_map(|line| line.chars())
                        .filter(|&c| c == '#')
                        .count()
                )
            } else {
                None
            }
        })
        .collect();

    let mut total = 0;
    for region in &puzzle[23.min(puzzle.len())..] {
        let parts: Vec<&str> = region.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let wxh = parts[0];
        let n_gifts = &parts[1..];

        // Parser width x height (format "12x5:")
        if let Some(dimensions) = wxh.strip_suffix(':') {
            if let Some((width_str, height_str)) = dimensions.split_once('x') {
                if let (Ok(width), Ok(height)) = (width_str.parse::<usize>(), height_str.parse::<usize>()) {
                    let area = width * height;

                    // Calculer la somme des gifts * sizes
                    let required_area: usize = n_gifts
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, count_str)| {
                            count_str.parse::<usize>().ok().map(|count| {
                                count * sizes.get(idx).unwrap_or(&0)
                            })
                        })
                        .sum();

                    if area >= required_area {
                        total += 1;
                    }
                }
            }
        }
    }
    println!("{}", total);
    total
}

#[cfg(test)]
mod tests {
    use crate::days_2025::day12::solve_day_12_2025;


    #[test]
    fn test_demo_part_1_12_2025() {
        let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;
        let result = solve_day_12_2025(input);
        assert_eq!(result, 2);
    }
}