use std::collections::HashMap;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day01.txt";

type InputType = (Vec<u64>, Vec<u64>);

fn part1((list1, list2): &InputType) -> Result<(), Error> {
    let mut list1 = list1.clone();
    let mut list2 = list2.clone();

    list1.sort_unstable();
    list2.sort_unstable();

    let answer: u64 = list1
        .into_iter()
        .zip(list2)
        .map(|(n1, n2)| n1.abs_diff(n2))
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2((list1, list2): &InputType) -> Result<(), Error> {
    let mut map: HashMap<u64, u64> = HashMap::new();
    for number in list2.iter().copied() {
        *map.entry(number).or_default() += 1;
    }

    let answer: u64 = list1
        .iter()
        .map(|number| map.get(number).map_or(0, |count| number * count))
        .sum();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .filter_map(|line| {
            let mut iter = line.split_whitespace();
            let fst: u64 = iter.next()?.parse().ok()?;
            let snd: u64 = iter.next()?.parse().ok()?;
            Some((fst, snd))
        })
        .unzip();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
