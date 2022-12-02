#[derive(Debug)]
pub enum RPSOption {
    Rock,
    Paper,
    Scissors,
}

impl RPSOption {
    pub fn get_from_string(input: &str) -> RPSOption {
        match input.chars().nth(0).unwrap() {
            'A' | 'X' => RPSOption::Rock,
            'B' | 'Y' => RPSOption::Paper,
            'C' | 'Z' => RPSOption::Scissors,
            _ => panic!("Unknown input {}", input.as_bytes()[0]),
        }
    }
}

#[derive(Debug)]
pub struct RPSGuess {
    guess: RPSOption,
    answer: RPSOption,
}

impl RPSGuess {
    pub fn new(input_pair: &str) -> Self {
        let mut guess: Option<RPSOption> = None;
        let mut answer: Option<RPSOption> = None;
        let input = input_pair.split(" ");
        input.for_each(|i| {
            if guess.is_some() {
                answer = Some(RPSOption::get_from_string(i.trim()))
            } else {
                guess = Some(RPSOption::get_from_string(i.trim()))
            }
        });
        return RPSGuess {
            guess: guess.unwrap(),
            answer: answer.unwrap(),
        };
    }

    pub fn cmp(&self) -> u8 {
        let type_points = match self.answer {
            RPSOption::Rock => 1,
            RPSOption::Paper => 2,
            RPSOption::Scissors => 3,
        };
        let round_points = match (&self.guess, &self.answer) {
            (RPSOption::Rock, RPSOption::Rock)
            | (RPSOption::Paper, RPSOption::Paper)
            | (RPSOption::Scissors, RPSOption::Scissors) => 3,

            (RPSOption::Rock, RPSOption::Paper)
            | (RPSOption::Paper, RPSOption::Scissors)
            | (RPSOption::Scissors, RPSOption::Rock) => 6,

            (RPSOption::Paper, RPSOption::Rock)
            | (RPSOption::Rock, RPSOption::Scissors)
            | (RPSOption::Scissors, RPSOption::Paper) => 0,
        };
        type_points + round_points
    }
}
