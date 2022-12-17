use std::{fs, ops};

fn main() {
    find_elves_carrying_the_most_calories(1);
    find_elves_carrying_the_most_calories(3);
    calculate_rock_paper_scissors_score_for_strategy_guide();
    calculate_rock_paper_scissors_score_for_corrected_strategy_guide();
    sum_rucksack_item_type_priorities();
    sum_group_item_type_priorities();
    sum_assignments_contained_in_pair_assignment();
    sum_overlapping_pair_assignments();
    get_top_crates_after_rearrangement_9000();
    get_top_crates_after_rearrangement_9001();
    find_start_of_packet_marker();
    find_start_of_message_marker();
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
    let score: u64 = input
        .lines()
        .map(|l| {
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
        })
        .sum();

    println!(
        "I followed the (corrected) strategy guide. My score is {}",
        score
    );
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
                _ => {
                    if inventory[item as usize] {
                        priority_sum += item as u64 + 1;
                        break;
                    }
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

struct AssignmentPair {
    left: ops::RangeInclusive<usize>,
    right: ops::RangeInclusive<usize>,
}

impl AssignmentPair {
    pub fn is_fully_contained(&self) -> bool {
        matches!(
            self.left.clone().all(|n| self.right.clone().contains(&n))
                || self.right.clone().all(|n| self.left.clone().contains(&n)),
            true
        )
    }

    pub fn overlaps(&self) -> bool {
        matches!(
            self.left.clone().any(|n| self.right.clone().contains(&n)),
            true
        )
    }
}

impl std::str::FromStr for AssignmentPair {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let assignments: Vec<_> = value
            .split(',')
            .map(|assign| {
                let bounds: Vec<_> = assign
                    .split('-')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                assert_eq!(bounds.len(), 2);
                bounds[0]..=bounds[1]
            })
            .collect();
        assert_eq!(assignments.len(), 2);
        Ok(AssignmentPair {
            left: assignments[0].clone(),
            right: assignments[1].clone(),
        })
    }
}

fn sum_assignments_contained_in_pair_assignment() {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let count = input
        .lines()
        .filter_map(|l| {
            l.parse::<AssignmentPair>()
                .unwrap()
                .is_fully_contained()
                .then_some(l)
        })
        .count();

    println!(
        "Found {} assignment pairs in which one range fully contains the other",
        count
    );
}

fn sum_overlapping_pair_assignments() {
    let input = fs::read_to_string("input/day4.txt").unwrap();
    let count = input
        .lines()
        .filter_map(|l| l.parse::<AssignmentPair>().unwrap().overlaps().then_some(l))
        .count();

    println!(
        "Found {} assignment pairs in which the ranges overlap",
        count
    );
}

fn get_top_crates_after_rearrangement_9000() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let lines = input.lines();

    // parse starting arrangement
    let mut init = false;
    let mut stacks = vec![];

    for line in lines {
        match line {
            l if l.trim_start().starts_with("[") => {
                for i in (0..l.len()).step_by(4) {
                    if stacks.len() < i / 4 + 1 {
                        stacks.push("".to_string());
                    }

                    let id = (&l[i+1..i+2]).trim();
                    if !id.is_empty() {
                        stacks[i / 4] += id;
                    }
                }
            },
            l if l.trim_start().starts_with("move") => {
                // parse instructions
                let tokens: Vec<_> = l.split_ascii_whitespace().collect();
                assert_eq!(tokens[0], "move");
                let num_crates: usize = tokens[1].parse().unwrap();
                assert_eq!(tokens[2], "from");
                let src_stack: usize = tokens[3].parse().unwrap();
                assert_eq!(tokens[4], "to");
                let dst_stack: usize = tokens[5].parse().unwrap();

                // perform instructions
                for _ in 0..num_crates {
                    let ch = stacks[src_stack - 1].pop().unwrap();
                    stacks[dst_stack - 1].push(ch);
                }
            },
            _ if !init => {
                // stacks were built in reverse; fix them
                for i in 0..stacks.len() {
                    stacks[i] = stacks[i].chars().rev().collect::<String>();
                }

                init = true;
            },
            _ => (),
        }
    }

    // get the top crate from each stack
    let mut top = "".to_string();
    for stack in stacks {
        top.push(stack.chars().last().unwrap());
    }

    println!("After rearrangement, the crates on top of each stack are {}", top);
}

fn get_top_crates_after_rearrangement_9001() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let lines = input.lines();

    // parse starting arrangement
    let mut init = false;
    let mut stacks = vec![];

    for line in lines {
        match line {
            l if l.trim_start().starts_with("[") => {
                for i in (0..l.len()).step_by(4) {
                    if stacks.len() < i / 4 + 1 {
                        stacks.push("".to_string());
                    }

                    let id = (&l[i+1..i+2]).trim();
                    if !id.is_empty() {
                        stacks[i / 4] += id;
                    }
                }
            },
            l if l.trim_start().starts_with("move") => {
                // parse instructions
                let tokens: Vec<_> = l.split_ascii_whitespace().collect();
                assert_eq!(tokens[0], "move");
                let num_crates: usize = tokens[1].parse().unwrap();
                assert_eq!(tokens[2], "from");
                let src_stack: usize = tokens[3].parse().unwrap();
                assert_eq!(tokens[4], "to");
                let dst_stack: usize = tokens[5].parse().unwrap();

                // perform instructions
                let stack = &mut stacks[src_stack - 1];
                let crates: String = stack.drain((stack.len() - num_crates)..).collect();
                stacks[dst_stack - 1] += &crates;
            },
            _ if !init => {
                // stacks were built in reverse; fix them
                for i in 0..stacks.len() {
                    stacks[i] = stacks[i].chars().rev().collect::<String>();
                }

                init = true;
            },
            _ => (),
        }
    }

    // get the top crate from each stack
    let mut top = "".to_string();
    for stack in stacks {
        top.push(stack.chars().last().unwrap());
    }

    println!("After rearrangement, the crates on top of each stack are {}", top);
}

fn find_start_of_packet_marker() {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let stream = input.as_bytes().to_vec();
    for (i, marker) in stream.windows(4).enumerate() {
        let mut marker = marker.to_vec();
        marker.sort();
        let ch = marker.iter().reduce(|last, ch| {
            match last {
                i if i == &0 || i == ch => &0,
                _ => ch,
            }
        }).unwrap();
        if *ch != 0 {
            println!("Found start-of-packet marker at {}", i + 4);
            break;
        }
    }
}

fn find_start_of_message_marker() {
    let input = fs::read_to_string("input/day6.txt").unwrap();
    let stream = input.as_bytes().to_vec();
    for (i, marker) in stream.windows(14).enumerate() {
        let mut marker = marker.to_vec();
        marker.sort();
        let ch = marker.iter().reduce(|last, ch| {
            match last {
                i if i == &0 || i == ch => &0,
                _ => ch,
            }
        }).unwrap();
        if *ch != 0 {
            println!("Found start-of-packet marker at {}", i + 14);
            break;
        }
    }
}
