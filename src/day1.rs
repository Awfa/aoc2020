use anyhow::Error;
use std::{collections::HashSet, num::ParseIntError};

pub fn input(input: &str) -> Result<Vec<u64>, ParseIntError> {
    input
        .split_ascii_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()
}

pub fn part1(numbers: &[u64]) -> Result<(), Error> {
    let mut number_set = HashSet::new();
    for number in numbers {
        let complement = 2020 - number;
        if number_set.contains(&complement) {
            println!("{} * {} = {}", number, complement, number * complement);
            return Ok(());
        } else {
            number_set.insert(number);
        }
    }

    println!("2-sum not found");

    Ok(())
}

pub fn part2(numbers: &[u64]) -> Result<(), Error> {
    const TARGET: u64 = 2020;

    let sorted_numbers = {
        let mut copy = numbers.to_vec();
        copy.sort();
        copy
    };

    for i in 0..sorted_numbers.len() - 2 {
        let first_number = sorted_numbers[i];
        if first_number >= TARGET {
            continue;
        }

        let mut search_window = &sorted_numbers[i + 1..sorted_numbers.len()];
        while search_window.len() > 1 {
            let second_number = search_window[0];
            let third_number = search_window[search_window.len() - 1];

            match (first_number + second_number + third_number).cmp(&TARGET) {
                std::cmp::Ordering::Less => {
                    search_window = &search_window[1..];
                }
                std::cmp::Ordering::Equal => {
                    println!(
                        "{} * {} * {} = {}",
                        first_number,
                        second_number,
                        third_number,
                        first_number * second_number * third_number
                    );

                    return Ok(());
                }
                std::cmp::Ordering::Greater => {
                    search_window = &search_window[..search_window.len() - 1]
                }
            }
        }
    }

    println!("3-sum not found");

    Ok(())
}
