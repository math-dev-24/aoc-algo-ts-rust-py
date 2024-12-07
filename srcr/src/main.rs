mod days;
mod utils;

use std::time::Instant;
use utils::input::Input;

use crate::days::day7::solve_day_7;

#[tokio::main]
async fn main() {
    let input = Input::new();
    let get_input = input.get_input(2024, 7).await;

    let start_time: Instant = Instant::now();
    solve_day_7(&get_input);
    println!("Temps du script {}s", start_time.elapsed().as_secs_f64());

}