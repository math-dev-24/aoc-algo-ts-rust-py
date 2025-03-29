
pub fn solve_day_2_part_1(data: &str) -> String {
    let keypad = [
        ['1', '2', '3'],
        ['4', '5', '6'],
        ['7', '8', '9']
    ];

    let mut row = 1;
    let mut col = 1;
    let mut code = String::new();

    for line in data.lines() {
        for c in line.chars() {
            match c {
                'U' => if row > 0 { row -= 1 },
                'D' => if row < 2 { row += 1 },
                'L' => if col > 0 { col -= 1 },
                'R' => if col < 2 { col += 1 },
                _ => {}
            }
        }

        code.push(keypad[row][col]);
    }
    println!("code: {}", code);

    code
}