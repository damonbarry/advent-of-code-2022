use std::fs;

fn main() {
    find_elves_carrying_the_most_calories(1);
    find_elves_carrying_the_most_calories(3);
    calculate_rock_paper_scissors_score_for_strategy_guide();
    calculate_rock_paper_scissors_score_for_corrected_strategy_guide();
}

fn find_elves_carrying_the_most_calories(num: usize) {
    let input = fs::read_to_string("input/day1.txt").unwrap();

    let mut lines = input.lines();
    let mut top: Vec<u64> = vec![];

    loop {
        let calories = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.parse::<u64>().unwrap())
            .reduce(|acc, cal| acc + cal);

        match calories {
            Some(c) => {
                let mut temp = Vec::from(top);
                temp.push(c);
                temp.sort_unstable_by(|a, b| b.cmp(a));
                temp.truncate(num);
                top = temp;
            }
            None => break,
        }
    }

    println!(
        "The {} elves carrying the most calories have {} calories in total",
        num,
        top.iter().sum::<u64>()
    );
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn new(code: &str) -> Self {
        match code {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Unknown move code '{}'", code),
        }
    }

    fn score(&self, other: &Move) -> u64 {
        match (self, other) {
            (Move::Rock, Move::Rock) => 1 + 3,         // draw
            (Move::Rock, Move::Paper) => 1 + 0,        // lose
            (Move::Rock, Move::Scissors) => 1 + 6,     // win
            (Move::Paper, Move::Rock) => 2 + 6,        // win
            (Move::Paper, Move::Paper) => 2 + 3,       // draw
            (Move::Paper, Move::Scissors) => 2 + 0,    // lose
            (Move::Scissors, Move::Rock) => 3 + 0,     // lose
            (Move::Scissors, Move::Paper) => 3 + 6,    // win
            (Move::Scissors, Move::Scissors) => 3 + 3, // draw
        }
    }
}

fn calculate_rock_paper_scissors_score_for_strategy_guide() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let score: u64 = input
        .lines()
        .map(|line| {
            let moves: Vec<_> = line.split(' ').map(|code| Move::new(code)).collect();
            assert_eq!(moves.len(), 2);
            moves[1].score(&moves[0])
        })
        .sum();

    println!("I followed the strategy guide. My score is {}", score);
}

fn calculate_rock_paper_scissors_score_for_corrected_strategy_guide() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let score: u64 = input.lines().map(|l| {
        match l {
            "A X" => 3 + 0, // Rock, lose => scissors
            "A Y" => 1 + 3, // Rock, draw => rock
            "A Z" => 2 + 6, // Rock, win => paper
            "B X" => 1 + 0, // Paper, lose => rock
            "B Y" => 2 + 3, // Paper, draw => paper
            "B Z" => 3 + 6, // Paper, win => scissors
            "C X" => 2 + 0, // Scissors, lose => paper
            "C Y" => 3 + 3, // Scissors, draw => scissors
            "C Z" => 1 + 6, // Scissors, win => rock
            _ => panic!("Unknown round combo '{}'", l),
        }
    }).sum();

    println!("I followed the (corrected) strategy guide. My score is {}", score);
}