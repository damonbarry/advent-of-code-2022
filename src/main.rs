use std::fs;

fn main() {
    find_elves_carrying_the_most_calories(1);
    find_elves_carrying_the_most_calories(3);
    calculate_rock_paper_scissors_score_for_strategy_guide();
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

fn calculate_rock_paper_scissors_score_for_strategy_guide() {
    let input = fs::read_to_string("input/day2.txt").unwrap();
    let scores: Vec<u64> = input.lines().map(|l| {
        let moves : Vec<_> = l.split(' ').collect();
        let move_scores: Vec<u64> = moves.iter().map(|mv| {
            match *mv {
                "A" | "X" => 1,
                "B" | "Y" => 2,
                "C" | "Z" => 3,
                _ => panic!("Unknown move code '{}'", mv),
            }
        }).collect();

        match (move_scores[0], move_scores[1]) {
            (1, 1) => 1 + 3, // Rock vs rock, draw
            (1, 2) => 2 + 6, // Rock vs paper, win
            (1, 3) => 3 + 0, // Rock vs scissors, lose
            (2, 1) => 1 + 0, // Paper vs rock, lose
            (2, 2) => 2 + 3, // Paper vs paper, draw
            (2, 3) => 3 + 6, // Paper vs scissors, win
            (3, 1) => 1 + 6, // Scissors vs rock, win
            (3, 2) => 2 + 0, // Scissors vs paper, lose
            (3, 3) => 3 + 3, // Scissors vs scissors, draw
            _ => panic!("Unknown round combo '{:?}'", (move_scores[0], move_scores[1])),
        }
    }).collect();

    println!("I followed the strategy guide. My score is {}", scores.iter().sum::<u64>());
}
