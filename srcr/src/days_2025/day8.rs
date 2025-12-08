use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn new(input: &str) -> Coord {
        let list: Vec<i64> = input
            .split(',')
            .map(|i| i.parse::<i64>().unwrap())
            .collect();

        if list.len() != 3 {
            panic!("Erreur : la coordonnée doit contenir exactement 3 valeurs !");
        }

        Coord {
            x: list[0],
            y: list[1],
            z: list[2],
        }
    }

    fn get_dist_squared(&self, versus: &Coord) -> i64 {
        let dx = versus.x - self.x;
        let dy = versus.y - self.y;
        let dz = versus.z - self.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
struct Edge {
    i: usize,
    j: usize,
    dist_sq: i64,
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let parent: Vec<usize> = (0..size).collect();
        let size: Vec<usize> = vec![1; size];
        UnionFind { parent, size }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.parent[a] != a {
            self.parent[a] = self.find(self.parent[a]);
        }
        self.parent[a]
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        // Union by size
        if self.size[root_a] < self.size[root_b] {
            self.parent[root_a] = root_b;
            self.size[root_b] += self.size[root_a];
        } else {
            self.parent[root_b] = root_a;
            self.size[root_a] += self.size[root_b];
        }
    }
}

pub fn solve_day_8(input: &str) {
    let mut list_coord: Vec<Coord> = vec![];

    for line in input.lines() {
        list_coord.push(Coord::new(line));
    }

    let n = list_coord.len();

    
    let list_coord_ref = &list_coord;

    let mut list_edges: Vec<Edge> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            (i + 1..n).into_par_iter().map(move |j| {
                let dist_sq = list_coord_ref[i].get_dist_squared(&list_coord_ref[j]);
                Edge { i, j, dist_sq }
            })
        })
        .collect();

    list_edges.sort_by_key(|e| e.dist_sq);

    // PARTIE 1 ----------------------------------------------------------
    println!("=== PARTIE 1 ===");
    let mut uf1 = UnionFind::new(n);
    connect_until(&mut uf1, &list_edges, |attempts, _| attempts == 1000);
    
    // Compter la taille de chaque circuit
    let mut circuit_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf1.find(i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }
    
    // Trier les tailles par ordre décroissant
    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort_by(|a, b| b.cmp(a));  // Ordre décroissant
    
    println!("Circuits : {:?}", circuit_sizes);
    println!("Tailles triées : {:?}", sizes);
    
    // Multiplier les 3 plus grandes tailles
    if sizes.len() >= 3 {
        let result1 = sizes[0] * sizes[1] * sizes[2];
        println!("Résultat partie 1 : {}", result1);
    } else {
        println!("Pas assez de circuits (seulement {})", sizes.len());
    }
    // PARTIE 2 ----------------------------------------------------------
    println!("\n=== PARTIE 2 ===");
    let mut uf2 = UnionFind::new(n);
    let last_edge = connect_until(&mut uf2, &list_edges, |_, components| components == 1);
    
    if let Some(edge) = last_edge {
        let coord_i = &list_coord[edge.i];
        let coord_j = &list_coord[edge.j];
        
        let result2 = coord_i.x * coord_j.x;
        
        println!("Dernière arête connectée : {} <-> {}", edge.i, edge.j);
        println!("Coord {} : ({}, {}, {})", edge.i, coord_i.x, coord_i.y, coord_i.z);
        println!("Coord {} : ({}, {}, {})", edge.j, coord_j.x, coord_j.y, coord_j.z);
        println!("Résultat partie 2 : {} * {} = {}", coord_i.x, coord_j.x, result2);
    } else {
        println!("Aucune arête connectée !");
    }
}


fn connect_until<'a>(
    uf: &mut UnionFind,
    list_edges: &'a [Edge],
    stop_condition: impl Fn(usize, usize) -> bool
) -> Option<&'a Edge> {
    let mut components = uf.parent.len();
    let mut attempts = 0;
    let mut last_edge = None;
    
    for edge in list_edges {
        if uf.find(edge.i) != uf.find(edge.j) {
            uf.union(edge.i, edge.j);
            components -= 1;
            last_edge = Some(edge);
        }
        attempts += 1;
        
        if stop_condition(attempts, components) {
            break;
        }
    }
    
    last_edge
}