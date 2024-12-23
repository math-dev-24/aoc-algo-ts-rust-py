mod days;
mod utils;

use std::time::Instant;
use utils::input::Input;
use crate::days::day23_p2::solve_day_23_p2;

#[tokio::main]
async fn main() {
    let input = Input::new();
    let year = 2024;
    let day = 23;
    println!("Récupération des données...[year:{},day:{}]", &year, &day);
    let get_input = input.get_input(year, day).await;
    println!("Récupération des données...OK");
    println!("Début du script...");
    let start_time: Instant = Instant::now();
    solve_day_23_p2(&get_input);
    println!("Temps du script {}s", start_time.elapsed().as_secs_f64());
}