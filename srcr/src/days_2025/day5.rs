pub fn solve_day_5 (input: &str) {

    let mut valid: usize = 0;

    let (list_ids, list_id) = clean_data(&input);
    let result_part_2 = part_2(&input);

    // part 1
    for id in list_id {
        for (min, max) in &list_ids {
            if id >= *min && id <= *max {
                valid +=1;
                break;
            }
        }
    }

    println!("Résultat : {}", valid);
    println!("Résultat 2 : {}", result_part_2);

}


fn part_2(input: &str) -> usize {
    let (list_ids, _) = clean_data(input);

    let mut intervals = list_ids;
    intervals.sort_by_key(|&(min, _)| min);

    let mut merged: Vec<(usize, usize)> = Vec::new();

    for (min, max) in intervals {
        if merged.is_empty() {
            merged.push((min, max));
        } else {
            let last_idx = merged.len() - 1;
            let (last_min, last_max) = merged[last_idx];

            if min <= last_max + 1 {
                merged[last_idx] = (last_min, last_max.max(max));
            } else {
                merged.push((min, max));
            }
        }
    }
    merged.iter().map(|(min, max)| max - min + 1).sum()
}

fn clean_data(input: &str)-> (Vec<(usize, usize)>, Vec<usize>) {

let group = input.split("\n\n").collect::<Vec<&str>>();

    let list_ids = group[0]
    .split("\n")
    .filter(|r| !r.trim().is_empty())
    .map(|r| {
        let (min, max) = r.split_once("-").unwrap();
        (min.trim().parse().unwrap(), max.trim().parse().unwrap())
    })
    .collect::<Vec<(usize, usize)>>();

    let list_id = group[1]
        .split("\n")
        .filter(|i| !i.trim().is_empty())
        .map(|i| i.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (list_ids, list_id)
}