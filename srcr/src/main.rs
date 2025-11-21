mod utils;
mod days_2023;

use std::time::Instant;
use utils::input::Input;

use crate::days_2023::day24;
use crate::utils::log::{LogDetail, LogLevel, Logger};

#[tokio::main]
async fn main() {
    let input = Input::new();
    // -------------------------------------------------------------------------------------

    let year = 2023;
    let day = 24;
    let mut logger = Logger::new(LogLevel::Debug);

    // -------------------------------------------------------------------------------------
    let info_start = format!("Récupération des données... - year:{} - day:{}", &year, &day);

    logger.post(LogDetail {
        msg: info_start,
        timestamp: Instant::now()
    });

    let get_input = match input.get_input(year, day).await {
        Ok(input) => input,
        Err(e) => {
            logger.post(LogDetail {
                msg: format!("Erreur : {}",e)
                ,timestamp: Instant::now()
            });
            return;
        }
    };

    logger.post(LogDetail {
        msg: "Récupération des données...OK".to_string(),
        timestamp: Instant::now()
    });

    logger.post(LogDetail {
        msg: format!("Input : {}", &get_input),
        timestamp: Instant::now()
    });

    logger.post(LogDetail {
        msg: "Script en cours...".to_string(),
        timestamp: Instant::now()
    });

    let start_time: Instant = Instant::now();

    // Script ACTION
    day24::solve_day4_2023(&get_input);
    // End Script ACTION

    logger.post(LogDetail {
        msg: format!("Temps du script {}s", start_time.elapsed().as_secs_f64()),
        timestamp: Instant::now()
    });


    logger.save().unwrap();
}