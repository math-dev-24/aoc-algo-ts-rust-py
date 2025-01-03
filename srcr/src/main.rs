mod utils;
mod days_2023;
mod days_2024;

use std::time::Instant;
use utils::input::Input;
use crate::days_2024::day15::solve_run_day_15;

#[tokio::main]
async fn main() {
    let input = Input::new();
    let year = 2024;
    let day = 15;
    println!("Récupération des données... - year:{} - day:{}", &year, &day);
    let get_input = input.get_input(year, day).await;
    println!("Récupération des données...OK");
    println!("Début du script...");
    let start_time: Instant = Instant::now();
    solve_run_day_15(&get_input);
    println!("Temps du script {}s", start_time.elapsed().as_secs_f64());
}