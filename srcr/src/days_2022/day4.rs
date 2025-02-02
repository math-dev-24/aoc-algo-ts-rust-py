
pub fn solve_day4(input: &str) -> usize {
    let lines = input.lines();
    let mut count: usize = 0;
    let mut count_2: usize = 0;

    for line in lines {
        let ranges: Vec<(usize, usize)> = line
            .split(",")
            .map(|range| {
                let bounds: Vec<usize> = range
                    .split("-")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect();

                (bounds[0], bounds[1])
            })
            .collect();

        let left = ranges[0];
        let right = ranges[1];

        if is_overlapping(left, right) {
            count += 1;
        }
        if is_in_range(left, right) || is_overlapping(left, right) {
            count_2 += 1;
        }else { println!("{:?}", ranges)}
    }
    println!("Partie 1: {}", count);
    println!("Partie 2: {}", count_2);
    0
}


fn is_overlapping(value_1: (usize, usize), value_2: (usize, usize)) -> bool {
    let (x1, x2) = value_1;
    let (y1, y2) = value_2;
    (x1 >= y1 && x1 <= y2 && x2 >= y1 && x2 <= y2) || (y1 >= x1 && y1 <= x2 && y2 >= x1 && y2 <= x2)
}

fn is_in_range(value_1: (usize, usize), value_2: (usize, usize)) -> bool {
    let (x1, x2) = value_1;
    let (y1, y2) = value_2;

    (x1 <= y1 && x2 <= y2 && x2 >= y1) || (y1 <= x1 && y2 <= x2 && y2 >= x1)

}
