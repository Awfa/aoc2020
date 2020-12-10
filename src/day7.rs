use std::collections::HashMap;

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::iterator,
    sequence::{preceded, terminated},
};
use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Walker;

type CountType = u64;

#[derive(Debug)]
struct Insides<'a> {
    count: CountType,
    color: &'a str,
}

fn get_insides(input: &str) -> nom::IResult<&str, Insides> {
    let (rest, number) = nom::character::complete::digit1(input)?;
    let number = CountType::from_str_radix(number, 10).unwrap();

    let (rest, color) = preceded(
        multispace1,
        terminated(take_until(" bag"), alt((tag(" bags"), tag(" bag")))),
    )(rest)?;
    Ok((
        rest,
        Insides {
            count: number,
            color,
        },
    ))
}

fn get_color(input: &str) -> nom::IResult<&str, &str> {
    const BAG_MIDDLE: &'static str = " bags contain ";
    let (rest, color) = terminated(take_until(BAG_MIDDLE), tag(BAG_MIDDLE))(input)?;

    Ok((rest, color))
}

pub fn input(input: &str) -> Result<DiGraphMap<&str, CountType>, anyhow::Error> {
    let mut graph = DiGraphMap::new();

    for line in input.lines() {
        let (rest, color) = get_color(line).map_err(|e| anyhow!("{}", e))?;
        graph.add_node(color);

        let mut inside_iterator =
            iterator(rest, terminated(get_insides, alt((tag(", "), tag(".")))));
        for inside in &mut inside_iterator {
            graph.add_node(inside.color);
            graph.add_edge(color, inside.color, inside.count);
        }
    }

    Ok(graph)
}

pub fn part1(input: &DiGraphMap<&str, CountType>) {
    let reversed_graph = petgraph::visit::Reversed(input);
    let dfs = petgraph::visit::Dfs::new(reversed_graph, "shiny gold");
    println!("Count = {}", dfs.iter(reversed_graph).count() - 1); // -1 so we don't count shiny gold itself
}

pub fn part2(input: &DiGraphMap<&str, CountType>) {
    let reversed_graph = petgraph::visit::Reversed(input);
    let topo = petgraph::visit::Topo::new(reversed_graph);
    let mut bag_counts = HashMap::new();
    for node in topo.iter(reversed_graph) {
        let inner_bags: CountType = input
            .edges(node)
            .map(|(_, neighbor, neighbor_count)| {
                // neighbor_count represents how many of 'neighbor' bags this node holds
                // we also add on the amount of bags the 'neighbor' bag type holds (* how many of 'neighbor' bags the node holds)
                neighbor_count
                    + neighbor_count * bag_counts.get(neighbor).cloned().unwrap_or_default()
            })
            .sum();
        bag_counts.insert(node, inner_bags);
    }
    println!("Count = {}", bag_counts.get("shiny gold").unwrap());
}

pub fn generate_bad_case_for_non_dynamic_programming_solution() {
    println!("shiny gold bags contain 1 A a bag, 1 A b bag.");
    ('A'..='Z')
        .chain('a'..='z')
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|cs| {
            for color in 'a'..='b' {
                println!(
                    "{} {} bags contain 1 {} a bag, 1 {} b bag.",
                    cs[0], color, cs[1], cs[1]
                );
            }
        });
    for color in 'a'..='b' {
        println!("z {} bags contain no other bags.", color);
    }
}
