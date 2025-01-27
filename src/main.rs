#![allow(unreachable_code)]

use std::env;

use color_eyre::eyre::Context;
use color_eyre::Report;

mod day01;
mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;
mod interval;
mod timing;

pub type Error = Report;

fn main() -> Result<(), Error> {
    color_eyre::install()?;

    let arg: usize = match env::args().nth(1) {
        Some(arg) => arg.parse().context("could not parse arg as a day number")?,
        None => {
            eprintln!("please enter the day number you want to run.");
            std::process::exit(1);
        }
    };

    match arg {
        1 => day01::run(),
        2 => day02::run(),
        // 3 => day03::run(),
        // 4 => day04::run(),
        // 5 => day05::run(),
        // 6 => day06::run(),
        // 7 => day07::run(),
        // 8 => day08::run(),
        // 9 => day09::run(),
        // 10 => day10::run(),
        // 11 => day11::run(),
        // 12 => day12::run(),
        // 13 => day13::run(),
        // 14 => day14::run(),
        // 15 => day15::run(),
        // 16 => day16::run(),
        // 17 => day17::run(),
        // 18 => day18::run(),
        // 19 => day19::run(),
        // 20 => day20::run(),
        // 21 => day21::run(),
        // 22 => day22::run(),
        // 23 => day23::run(),
        // 24 => day24::run(),
        // 25 => day25::run(),
        n if (1..=25).contains(&n) => {
            println!("this day has not (yet) been solved !");
            Ok(())
        }
        _ => {
            eprintln!("invalid day number, it must be within the range [1,24].");
            std::process::exit(1);
        }
    }
}
