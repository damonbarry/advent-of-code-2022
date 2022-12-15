use std::fs;

fn main() {
    find_elves_carrying_the_most_calories(1);
    find_elves_carrying_the_most_calories(3);
    calculate_rock_paper_scissors_score_for_strategy_guide();
    calculate_rock_paper_scissors_score_for_corrected_strategy_guide();
    sum_rucksack_item_type_priorities();
    sum_group_item_type_priorities();
    sum_assignments_contained_in_pair_assignment();
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

fn sum_rucksack_item_type_priorities() {
    let input = fs::read_to_string("input/day3.txt").unwrap();
    let mut priority_sum = 0u64;
    for line in input.lines() {
        let compartment_size = line.len() / 2;
        let mut inventory = [false; 52];
        for (i, item) in line.chars().enumerate() {
            let item = match item {
                c if c.is_ascii_lowercase() => c as u32 - 'a' as u32,
                c if c.is_ascii_uppercase() => c as u32 - 'A' as u32 + 26,
                _ => panic!("Invalid rucksack item type '{}'", item),
            };
            match i {
                i if i < compartment_size => inventory[item as usize] = true,
                _ => if inventory[item as usize] {
                    priority_sum += item as u64 + 1;
                    break;
                }
            }
        }
    }

    println!("Sum of rucksack item type priorities is {}", priority_sum);
}

fn sum_group_item_type_priorities() {
    let mut priority_sum = 0u64;
    let input = fs::read_to_string("input/day3.txt").unwrap();
    for group in input.lines().collect::<Vec<_>>().as_slice().chunks(3) {
        let mut inventories = [[false; 52]; 3];
        for (i, rucksack) in group.iter().enumerate() {
            for item in rucksack.chars() {
                let item = match item {
                    c if c.is_ascii_lowercase() => c as u32 - 'a' as u32,
                    c if c.is_ascii_uppercase() => c as u32 - 'A' as u32 + 26,
                    _ => panic!("Invalid rucksack item type '{}'", item),
                };

                inventories[i][item as usize] = true;
            }
        }

        let mut group_item = 0usize;
        for n in 0..52 {
            if inventories[0][n] == true && inventories[1][n] == true && inventories[2][n] == true {
                group_item = n + 1;
            }
        }

        priority_sum += group_item as u64;
    }

    println!("Sum of group item type priorities is {}", priority_sum);
}

fn sum_assignments_contained_in_pair_assignment() {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let count = input.lines().filter_map(|l| {
        let elves: Vec<_> = l.split(',').collect();
        assert_eq!(elves.len(), 2);

        let ranges: Vec<_> = elves.iter().map(|elf| {
            let range: Vec<_> = elf.split('-').map(|n| n.parse::<usize>().unwrap()).collect();
            assert_eq!(range.len(), 2);
            range[0]..=range[1]
        }).collect();

        let (left, right) = (&ranges[0], &ranges[1]);

        match left.clone().all(|n| right.clone().contains(&n)) || right.clone().all(|n| left.clone().contains(&n)) {
            true => Some(l),
            false => None,
        }
    }).count();

    println!("Found {} assignment pairs in which one range fully contains the other", count);
}