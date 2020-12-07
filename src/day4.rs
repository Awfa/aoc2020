use std::collections::HashMap;

use anyhow::anyhow;

pub fn input<'a>(input: &'a str) -> Result<Vec<HashMap<&'a str, &'a str>>, anyhow::Error> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .split_whitespace()
                .map(|entry| {
                    entry
                        .find(':')
                        .map(|idx| entry.split_at(idx))
                        .map(|(key, value)| (key, &value[1..]))
                })
                .collect::<Option<HashMap<&'a str, &'a str>>>()
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(anyhow!("Error collecting input"))
}

pub fn part1<'a>(input: &[HashMap<&'a str, &'a str>]) {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let answer = input
        .iter()
        .filter(|passport| required_fields.iter().all(|rf| passport.contains_key(rf)))
        .count();
    println!("# of passports meeting requirements: {}", answer);
}

fn validate_year(input: &str, min: usize, max: usize) -> bool {
    input.len() == 4
        && usize::from_str_radix(input, 10)
            .map(|v| min <= v && v <= max)
            .unwrap_or(false)
}

fn read_height(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let (input, number) = nom::bytes::complete::take_while(|b: char| b.is_numeric())(input)?;
    let (input, suffix) = nom::bytes::complete::take_while(|b: char| b.is_alphabetic())(input)?;
    Ok((input, (number, suffix)))
}
fn validate_height(input: &&str) -> bool {
    if let Ok((rest, (number, suffix))) = read_height(input) {
        if !rest.is_empty() {
            return false;
        }

        if let Ok(height) = u32::from_str_radix(number, 10) {
            if suffix == "cm" {
                150 <= height && height <= 193
            } else if suffix == "in" {
                59 <= height && height <= 76
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn part2<'a>(input: &[HashMap<&'a str, &'a str>]) {
    let valid_eyecolors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let answer = input
        .iter()
        .filter(|passport| {
            passport
                .get("byr")
                .map(|v| validate_year(v, 1920, 2002))
                .unwrap_or(false)
                && passport
                    .get("iyr")
                    .map(|v| validate_year(v, 2010, 2020))
                    .unwrap_or(false)
                && passport
                    .get("eyr")
                    .map(|v| validate_year(v, 2020, 2030))
                    .unwrap_or(false)
                && passport.get("hgt").map(validate_height).unwrap_or(false)
                && passport
                    .get("hcl")
                    .map(|v| {
                        v.len() == 7
                            && v.starts_with('#')
                            && v[1..].chars().all(|c| c.is_ascii_hexdigit())
                    })
                    .unwrap_or(false)
                && passport
                    .get("ecl")
                    .map(|v| valid_eyecolors.contains(v))
                    .unwrap_or(false)
                && passport
                    .get("pid")
                    .map(|v| v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()))
                    .unwrap_or(false)
        })
        .count();
    println!("# of passports meeting requirements: {}", answer);
}
