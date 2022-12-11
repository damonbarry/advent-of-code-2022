use std::{cmp, fs};

fn main() {
    find_the_elf_carrying_the_most_calories();
}

fn find_the_elf_carrying_the_most_calories() {
    let input = fs::read_to_string("input/day1.txt").unwrap();

    let mut lines = input.lines();
    let mut most = 0u64;

    loop {
        let calories = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.parse::<u64>().unwrap())
            .reduce(|acc, cal| acc + cal);

        match calories {
            Some(c) => most = cmp::max(most, c),
            None => break,
        }
    }

    println!("The elf carrying the most calories has {} calories", most);
}