use std::sync::Arc;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn new(input: &str) -> Self {
        let res = input.split(",")
        .map(|n| {
            n.parse::<i64>().unwrap()
        }).collect::<Vec<i64>>();

        if res.len() != 2 {
            println!("{}", input);
            panic!("Invalide Coordonnées !");
        }
        Coord { x: res[0], y: res[1] }
    }

    fn get_size(&self, other: &Coord) -> i64 {
        let dif_x = (self.x - other.x).abs() + 1;
        let dif_y = (self.y - other.y).abs() + 1;
        dif_x * dif_y
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

// Vérifie si un rectangle intersecte avec les arêtes du polygone
fn rectangles_intersects_edges(min_x: i64, min_y: i64, max_x: i64, max_y: i64, edges: &[Edge]) -> bool {
    for edge in edges {
        let i_min_x = edge.x1.min(edge.x2);
        let i_max_x = edge.x1.max(edge.x2);
        let i_min_y = edge.y1.min(edge.y2);
        let i_max_y = edge.y1.max(edge.y2);

        // Vérifie si les bounding boxes se chevauchent
        if min_x < i_max_x && max_x > i_min_x && min_y < i_max_y && max_y > i_min_y {
            return true;
        }
    }
    false
}

// Calcule la distance de Manhattan entre deux points
fn manhattan_distance(a: &Coord, b: &Coord) -> i64 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn solve_part1(red_tiles: &[Coord]) -> i64 {
    let len_list = red_tiles.len();

    // Paralléliser avec rayon
    (0..len_list)
        .into_par_iter()
        .flat_map(|idx_1| {
            (idx_1 + 1..len_list)
                .into_par_iter()
                .map(move |idx_2| {
                    let corner_1 = &red_tiles[idx_1];
                    let corner_2 = &red_tiles[idx_2];
                    corner_1.get_size(corner_2)
                })
        })
        .max()
        .unwrap_or(0)
}

fn solve_part2(red_tiles: &[Coord]) -> i64 {
    // Construire les arêtes du polygone
    let mut edges = Vec::new();
    for i in 0..red_tiles.len() - 1 {
        edges.push(Edge {
            x1: red_tiles[i].x,
            y1: red_tiles[i].y,
            x2: red_tiles[i + 1].x,
            y2: red_tiles[i + 1].y,
        });
    }
    // Fermer le polygone
    let last_idx = red_tiles.len() - 1;
    edges.push(Edge {
        x1: red_tiles[last_idx].x,
        y1: red_tiles[last_idx].y,
        x2: red_tiles[0].x,
        y2: red_tiles[0].y,
    });

    let edges = Arc::new(edges);
    let len_list = red_tiles.len();

    // Générer toutes les paires avec leur aire potentielle
    let mut pairs: Vec<(usize, usize, i64)> = Vec::new();
    for idx_1 in 0..len_list {
        for idx_2 in idx_1 + 1..len_list {
            let from_tile = &red_tiles[idx_1];
            let to_tile = &red_tiles[idx_2];

            let area = from_tile.get_size(to_tile);
            pairs.push((idx_1, idx_2, area));
        }
    }

    // Trier par aire décroissante
    pairs.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    println!("Vérification des {} paires de rectangles...", pairs.len());

    // Utiliser un AtomicI64 pour suivre le maximum trouvé
    use std::sync::atomic::{AtomicI64, Ordering};
    let max_found = Arc::new(AtomicI64::new(0));

    pairs
        .par_iter()
        .for_each(|(idx_1, idx_2, potential_area)| {
            // Early exit : si l'aire potentielle est <= au max trouvé, ignorer
            let current_max = max_found.load(Ordering::Relaxed);
            if *potential_area <= current_max {
                return;
            }

            let from_tile = &red_tiles[*idx_1];
            let to_tile = &red_tiles[*idx_2];

            let min_x = from_tile.x.min(to_tile.x);
            let max_x = from_tile.x.max(to_tile.x);
            let min_y = from_tile.y.min(to_tile.y);
            let max_y = from_tile.y.max(to_tile.y);

            // Optimisation : vérifier la distance de Manhattan au carré
            let manhattan = manhattan_distance(from_tile, to_tile);
            if manhattan * manhattan <= current_max {
                return;
            }

            // Vérifier si le rectangle intersecte avec les arêtes
            if !rectangles_intersects_edges(min_x, min_y, max_x, max_y, &edges) {
                // Mettre à jour le maximum
                let mut old_max = max_found.load(Ordering::Relaxed);
                while *potential_area > old_max {
                    match max_found.compare_exchange_weak(
                        old_max,
                        *potential_area,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break,
                        Err(x) => old_max = x,
                    }
                }
            }
        });

    max_found.load(Ordering::Relaxed)
}

pub fn solve_day_9(input: &str) {

    let list_pos = input.lines()
        .filter_map(|l| {
            if !l.is_empty() {
                Some(Coord::new(l))
            } else {
                None
            }
        })
        .collect::<Vec<Coord>>();

    println!("Red tiles: {:?}\n", list_pos);

    let part1 = solve_part1(&list_pos);
    println!("Part 1 - Max area: {}", part1);

    let part2 = solve_part2(&list_pos);
    println!("Part 2 - Max area (red/green only): {}", part2);
}