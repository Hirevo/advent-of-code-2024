use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day02.txt";

type InputType = Vec<Vec<u64>>;

fn is_safe(levels: &[u64]) -> bool {
    if !levels.windows(2).all(|w| w[1] > w[0]) && !levels.windows(2).all(|w| w[1] < w[0]) {
        return false;
    }
    levels.windows(2).all(|w| {
        let diff = w[1].abs_diff(w[0]);
        diff >= 1 && diff <= 3
    })
}

fn check_windows_ignoring<T, F>(levels: &[T], ignored: usize, predicate: F) -> bool
where
    T: Copy,
    F: Fn(T, T) -> bool,
{
    let mut idx1 = 0;
    let mut idx2 = 1;

    if idx1 == ignored {
        idx1 += 1;
        idx2 += 1;
    }

    if idx2 == ignored {
        idx2 += 1;
    }

    while idx2 < levels.len() {
        if !predicate(levels[idx1], levels[idx2]) {
            return false;
        }

        idx1 += 1;
        idx2 += 1;
        if idx1 == ignored {
            idx1 += 1;
        }
        if idx2 == ignored {
            idx2 += 1;
        }
    }

    true
}

fn is_safe_ignoring(levels: &[u64], ignored: usize) -> bool {
    if !check_windows_ignoring(levels, ignored, |n1, n2| n1 > n2)
        && !check_windows_ignoring(levels, ignored, |n1, n2| n1 < n2)
    {
        return false;
    }
    check_windows_ignoring(levels, ignored, |n1, n2| {
        let diff = n1.abs_diff(n2);
        diff >= 1 && diff <= 3
    })
}

fn is_safe_with_tolerance(levels: &[u64]) -> bool {
    is_safe(levels) || (0..levels.len()).any(|ignored| is_safe_ignoring(levels, ignored))
}

fn part1(input: &InputType) -> Result<(), Error> {
    let answer = input.iter().filter(|levels| is_safe(levels)).count();
    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let answer = input
        .iter()
        .filter(|levels| is_safe_with_tolerance(levels))
        .count();
    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: InputType = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|level| level.parse().ok())
                .collect()
        })
        .collect();

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
