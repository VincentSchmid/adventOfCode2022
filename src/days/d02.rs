// https://adventofcode.com/2022/day/2

const WINNER_SCORE: u32 = 6;
const LOSER_SCORE: u32 = 0;
const TIE_SCORE: u32 = 3;

const ROCK_SCORE : u32 = 1;
const PAPER_SCORE : u32 = 2;
const SCISSORS_SCORE : u32 = 3;


fn map_char_to_rps_by_outcome(opponent: &str, outcome: &str) -> (RPS, RPS) {
    let opponent_move = match opponent {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissors,
        _ => panic!("Invalid input"),
    };

    let outcome = match outcome {
        "X" => Winner::Opponent,
        "Y" => Winner::Tie,
        "Z" => Winner::Me,
        _ => panic!("Invalid input"),
    };

    let my_move = match (opponent_move, outcome) {
        (RPS::Rock, Winner::Me) => RPS::Paper,
        (RPS::Rock, Winner::Opponent) => RPS::Scissors,
        (RPS::Rock, Winner::Tie) => RPS::Rock,
        (RPS::Paper, Winner::Me) => RPS::Scissors,
        (RPS::Paper, Winner::Opponent) => RPS::Rock,
        (RPS::Paper, Winner::Tie) => RPS::Paper,
        (RPS::Scissors, Winner::Me) => RPS::Rock,
        (RPS::Scissors, Winner::Opponent) => RPS::Paper,
        (RPS::Scissors, Winner::Tie) => RPS::Scissors,
    };

    (opponent_move, my_move)
}

pub fn solve(_input: String) -> (String, String) {
    println!("Day 2");
    println!("Part 1");
    let games: Vec<Game> = _input.split("\r\n").map(|s| Game::of(s)).collect();
    let total_score = games.iter().map(|g| g.score).sum::<u32>();

    println!("{:?}", games);
    ("".into(), total_score.to_string())
}

#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Winner {
    Opponent,
    Me,
    Tie,
}

#[derive(Debug)]
struct Game {
    score: u32,
}

impl Game {
    pub fn new(opponent: RPS, me: RPS) -> Self {
        let winner: Winner = Self::get_winner(&opponent, &me);
        let score: u32 = Self::get_score(&me, &winner);
        Self { score }
    }

    pub fn of(input: &str) -> Self {
        let play_chars= input
            .split(" ")
            .collect::<Vec<&str>>();

        let plays = map_char_to_rps_by_outcome(play_chars[0], play_chars[1]);
        Self::new(plays.0.clone(), plays.1.clone())
    }

    fn get_winner(opp: &RPS, me: &RPS) -> Winner {
        match (opp, me) {
            (RPS::Rock, RPS::Rock) => Winner::Tie,
            (RPS::Rock, RPS::Paper) => Winner::Me,
            (RPS::Rock, RPS::Scissors) => Winner::Opponent,
            (RPS::Paper, RPS::Rock) => Winner::Opponent,
            (RPS::Paper, RPS::Paper) => Winner::Tie,
            (RPS::Paper, RPS::Scissors) => Winner::Me,
            (RPS::Scissors, RPS::Rock) => Winner::Me,
            (RPS::Scissors, RPS::Paper) => Winner::Opponent,
            (RPS::Scissors, RPS::Scissors) => Winner::Tie,
        }
    }

    fn get_score(me: &RPS, winner: &Winner) -> u32 {
        let score = match me {
            RPS::Rock => ROCK_SCORE,
            RPS::Paper => PAPER_SCORE,
            RPS::Scissors => SCISSORS_SCORE,
        };

        score + (match winner {
            Winner::Me => WINNER_SCORE,
            Winner::Opponent => LOSER_SCORE,
            Winner::Tie => TIE_SCORE,
        })
    }
}
