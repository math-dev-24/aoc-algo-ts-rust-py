use regex::Regex;

struct RegexStruct<'a> {
    i: usize,
    value: &'a str,
}

pub fn solve_day_3() {
    let test: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let re_mull = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_action = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut score: u32 = 0;
    let mut state: bool = true;

    let mut matches: Vec<RegexStruct> = Vec::new();

    for action in re_action.find_iter(test) {
        matches.push(RegexStruct {
            i: action.start(),
            value: action.as_str(),
        });
    }
    for mull in re_mull.find_iter(test) {
        matches.push(RegexStruct {
            i: mull.start(),
            value: mull.as_str(),
        });
    }

    matches.sort_by_key(|m| m.i);

    for m in matches {
        if m.value == "do()" {
            state = true;
        }else if m.value == "don't()" {
            state = false
        }else if m.value.starts_with("mul(") && state {
            let caps = re_mull.captures(test).unwrap();
            let num1: u32 = caps[1].parse().unwrap();
            let num2: u32 = caps[2].parse().unwrap();
            score += num1 * num2;
        }
    }
    println!("Score : {}", score);
}