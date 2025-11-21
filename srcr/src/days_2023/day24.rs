use itertools::Itertools;

#[derive(Debug)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

pub fn solve_day4_2023(input: &str) {
    let hailstones = parse_hailstones(input);

    let min_xy = 200000000000000.0;
    let max_xy = 400000000000000.0;

    let mut count = 0;

    for pair in hailstones.iter().combinations(2) {
        if let Some((x, y)) = intersect_2d(pair[0], pair[1]) {
            if x >= min_xy && x <= max_xy && y >= min_xy && y <= max_xy {
                count += 1;
            }
        }
    }
    println!("Nombre d'intersections dans la zone : {}", count);
}

fn parse_hailstones(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();

            let p: Vec<f64> = pos.split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect();

            let v: Vec<f64> = vel.split(',')
                .map(|v| v.trim().parse().unwrap())
                .collect();

            Hailstone {
                px: p[0],
                py: p[1],
                pz: p[2],
                vx: v[0],
                vy: v[1],
                vz: v[2],
            }
        })
        .collect()
}


fn intersect_2d(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    // On veut résoudre :
    // A + tA * VA = B + tB * VB

    let dx = b.px - a.px;
    let dy = b.py - a.py;

    let det = a.vx * (-b.vy) - a.vy * (-b.vx);

    if det.abs() < 1e-12 {
        return None;
    }

    let t_a = (dx * (-b.vy) - dy * (-b.vx)) / det;
    let t_b = (a.vx * dy - a.vy * dx) / det;

    if t_a < 0.0 || t_b < 0.0 {
        return None;
    }

    let x = a.px + a.vx * t_a;
    let y = a.py + a.vy * t_a;

    Some((x, y))
}