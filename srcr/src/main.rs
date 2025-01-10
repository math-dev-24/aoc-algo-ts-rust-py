mod utils;
mod days_2021;

use std::time::Instant;
use utils::input::Input;
use crate::days_2021::day2::solve_day2;

#[tokio::main]
async fn main() {

    let input = Input::new();
    let year = 2021;
    let day = 2;
    println!("Récupération des données... - year:{} - day:{}", &year, &day);
    let get_input = input.get_input(year, day).await;
    println!("Récupération des données...OK");
    println!("Début du script...");
    let start_time: Instant = Instant::now();
    solve_day2(&get_input);
    println!("Temps du script {}s", start_time.elapsed().as_secs_f64());
}