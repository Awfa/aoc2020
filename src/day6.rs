use std::collections::HashMap;

pub fn part1(input: &str) {
    let sum: usize = input
        .split("\n\n")
        .map(str::trim)
        .map(|lines| {
            lines
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<std::collections::HashSet<_>>()
                .len()
        })
        .sum();
    println!("{}", sum);
}

pub fn part2(input: &str) {
    let sum: usize = input
        .split("\n\n")
        .map(str::trim)
        .map(|lines| {
            let (map, num_lines) = lines.chars().fold(
                (HashMap::<char, usize>::new(), 1),
                |(mut state, mut num_lines), c| {
                    if c.is_alphabetic() {
                        *state.entry(c).or_default() += 1;
                    } else if c.is_whitespace() {
                        num_lines += 1;
                    } else {
                        panic!("Unexpected character: {}", c);
                    }

                    (state, num_lines)
                },
            );
            let agreed_on_questions = map.iter().filter(|(_, v)| **v == num_lines).count();
            agreed_on_questions
        })
        .sum();
    println!("{}", sum);
}
