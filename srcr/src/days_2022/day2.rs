#[derive(Eq, PartialEq, Debug)]
enum Action { Rock, Paper, Scissors }

struct Round {
    opponent_letter: String,
    opponent: Action,
    me_letter: String,
    me: Action
}

impl Round {
    fn new(opponent: &str, me: &str) -> Self{
        Round {
            opponent_letter: opponent.to_string(),
            opponent: Round::get_action(opponent).unwrap(),
            me_letter: me.to_string(),
            me: Round::get_action(me).unwrap()
        }
    }
    fn get_me_point_part_1(&self) -> usize {
        let point_action: usize = Round::get_point_action(&self.me);
        let score_win: usize = 6 + point_action;
        if self.opponent == Action::Paper && self.me == Action::Scissors {
            return score_win;
        } else if self.opponent == Action::Rock && self.me == Action::Paper {
            return score_win;
        } else if self.opponent == Action::Scissors && self.me == Action::Rock {
            return score_win;
        } else if self.opponent == self.me {
            return 3 + point_action;
        }
        point_action
    }

    fn get_me_point_part_2(&self) -> usize {
        match self.me_letter.as_str() {
            "X" => {
                Round::get_point_action_relative(&self.opponent, false)
            }
            "Y" => {
                Round::get_point_action(&self.opponent) + 3
            }

            "Z" => {
                Round::get_point_action_relative(&self.opponent, true) + 6
            }
            _ => {
                println!("Commande introuvable");
                0
            }
        }
    }

    fn get_point_action(action: &Action) -> usize {
        match action {
            Action::Scissors => 3,
            Action::Paper => 2,
            Action::Rock => 1
        }
    }

    fn get_action(letter: &str) -> Result<Action, &'static str> {
        match letter {
            "A" | "X" => Ok(Action::Rock),
            "B" | "Y" => Ok(Action::Paper),
            "C" | "Z" => Ok(Action::Scissors),
            _ => Err("Introuvable !")
        }
    }

    fn get_point_action_relative(action: &Action, is_win: bool) -> usize {
        match (action, is_win) {
            (Action::Rock, true) => 2,
            (Action::Rock, false) => 3,
            (Action::Scissors, true) => 1,
            (Action::Scissors, false) => 2,
            (Action::Paper, true) => 3,
            (Action::Paper, false) => 1,
        }
    }
}




pub fn solve_day_2(input: &str) -> u32 {

    let lines = input.trim().lines();

    let mut total_part_1: usize = 0;
    let mut total_part_2: usize = 0;

    for line in lines {
        let value = line.split_whitespace().collect::<Vec<&str>>();
        let round = Round::new( value[0], value[1]);
        total_part_1 += round.get_me_point_part_1();
        total_part_2 += round.get_me_point_part_2();
    }
    println!("Partie 1 : {}", total_part_1);
    println!("Partie 2 : {}", total_part_2);
    1
}

