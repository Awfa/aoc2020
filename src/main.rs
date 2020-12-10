use anyhow::Error;
use aoc2020::*;

fn main() -> Result<(), Error> {
    // let input = std::fs::read_to_string("inputs/day01.txt")?;
    // let input = day1::input(&input)?;
    // day1::part1(&input)?;
    // day1::part2(&input)?;

    // let input = std::fs::read_to_string("inputs/day02.txt")?;
    // let input = day2::input(&input).unwrap();
    // day2::part1(&input);
    // day2::part2(&input);

    // let input = std::fs::read_to_string("inputs/day03.txt")?;
    // let input = day3::input(&input).unwrap();
    // day3::part1(&input);
    // day3::part2(&input);
    // Ok(())

    // let input = std::fs::read_to_string("inputs/day04.txt")?;
    // let input = day4::input(&input).unwrap();
    // day4::part1(&input);
    // day4::part2(&input);

    // let input = std::fs::read_to_string("inputs/day05.txt")?;
    // day5::part1(&input);
    // day5::part2(&input);

    // let input = std::fs::read_to_string("inputs/day06.txt")?;
    // day6::part1(&input);
    // day6::part2(&input);

    let input = std::fs::read_to_string("inputs/day07.txt")?;
    let input = day7::input(&input).unwrap();
    day7::part1(&input);
    day7::part2(&input);
    Ok(())
}
