
pub fn solve_d1(input: &str) -> u32 {
    let lines = input.split("\n\n");

    let mut max_part_1: usize = 0;
    let mut list_part_1: Vec<usize> = Vec::new();


    for elf in lines {
        let calories: usize = part1(elf);
        list_part_1.push(calories);
        if calories > max_part_1 {
            max_part_1 = calories;
        }
    }
    println!("Partie 1 : {} calorie", max_part_1);

    list_part_1.sort_by( |a, b| b.cmp(a));

    let part_2: usize = list_part_1[0..3].iter().sum();

    println!("Partie 2 : {:?}", part_2);

    2
}

fn part1(elf: &str) -> usize {

    elf
    .split("\n")
    .map(|i| i.parse::<usize>().unwrap_or(0))
    .sum()
}