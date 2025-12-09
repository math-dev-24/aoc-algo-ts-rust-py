
#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(input: &str) -> Self {
        let res = input.split(",")
        .map(|n| {
            n.parse::<usize>().unwrap()
        }).collect::<Vec<usize>>();

        if res.len() != 2 {
            println!("{}", input);
            panic!("Invalide Coordonnées !");
        }
        Coord { x: res[0], y: res[1] }
    }

    fn get_size(&self, other: &Coord) -> usize {
        let dif_x = self.x.abs_diff(other.x) + 1;
        let dif_y = self.y.abs_diff(other.y) + 1;
        dif_x * dif_y
    }
}

pub fn solve_day_9(input: &str) {
    let demo = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let list_pos = demo.lines()
    .filter_map(|l| {
        if !l.is_empty() {
            Some(Coord::new(l))
        } else {
            None
        }
    })
    .collect::<Vec<Coord>>();

    let len_list = list_pos.len();

    println!("{:?}", list_pos);

    let mut max: usize = 0;

    for idx_1 in  0..len_list {
        for idx_2 in idx_1+1..len_list {
            let corner_1 = &list_pos[idx_1];
            let corner_2 = &list_pos[idx_2];
            let dist = corner_1.get_size(&corner_2);
            if dist > max { max = dist};
        }
    }

    println!("Max : {}", max);

}