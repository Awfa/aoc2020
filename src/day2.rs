pub struct Entry<'a> {
    range_low: usize,
    range_high: usize,
    letter: char,
    string: &'a str,
}

fn read_base_10_number(input: &str) -> Result<usize, std::num::ParseIntError> {
    usize::from_str_radix(input, 10)
}

fn take_hypenated_range(input: &str) -> nom::IResult<&str, (usize, usize)> {
    let (rest, range_low) =
        nom::combinator::map_res(nom::character::complete::digit1, read_base_10_number)(input)?;
    let (rest, _) = nom::bytes::complete::tag("-")(rest)?;
    let (rest, range_high) =
        nom::combinator::map_res(nom::character::complete::digit1, read_base_10_number)(rest)?;
    Ok((rest, (range_low, range_high)))
}

fn take_entry(input: &str) -> nom::IResult<&str, Entry> {
    let (rest, (range_low, range_high)) = take_hypenated_range(input)?;
    let (rest, _) = nom::bytes::complete::tag(" ")(rest)?;

    let (rest, letter) = nom::character::complete::anychar(rest)?;
    let (rest, _) = nom::bytes::complete::tag(": ")(rest)?;

    let string = rest;
    Ok((
        rest,
        Entry {
            range_low,
            range_high,
            letter,
            string,
        },
    ))
}

pub fn input<'a>(input: &'a str) -> Result<Vec<Entry<'a>>, anyhow::Error> {
    input
        .lines()
        .map(|line| -> Result<Entry, anyhow::Error> {
            take_entry(line)
                .map(|(_, e)| e)
                .map_err(|e| anyhow::anyhow!("Entry parsing error: {}", e.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn part1<'a>(input: &[Entry<'a>]) {
    let count = input
        .iter()
        .filter(|entry| {
            let matching_count = entry.string.chars().filter(|c| *c == entry.letter).count();
            entry.range_low <= matching_count && matching_count <= entry.range_high
        })
        .count();
    println!("Count of valid passwords: {}", count);
}

pub fn part2<'a>(input: &[Entry<'a>]) {
    let count = input
        .iter()
        .filter(|entry| {
            entry
                .string
                .char_indices()
                .filter(|(idx, c)| {
                    (idx + 1 == entry.range_low || idx + 1 == entry.range_high)
                        && *c == entry.letter
                })
                .count()
                == 1
        })
        .count();
    println!("Count of valid passwords: {}", count);
}
