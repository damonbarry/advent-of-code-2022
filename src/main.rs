use std::{cmp, fs};

fn main() {
    find_elves_carrying_the_most_calories(1);
    find_elves_carrying_the_most_calories(3);
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
