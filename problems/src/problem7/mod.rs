use std::collections::HashMap;

use failure::Error;
use utils::{result, RetTypes};

#[derive(Debug)]
struct BagNameCount {
    name: String,
    count: usize,
}

fn first_star_helper(all: &HashMap<String, Vec<BagNameCount>>, bags: &[BagNameCount]) -> bool {
    for bag in bags {
        if bag.name == "shiny gold" {
            return true;
        }

        if first_star_helper(all, all.get(&bag.name).unwrap()) {
            return true;
        }
    }

    false
}

fn first_star(all: &HashMap<String, Vec<BagNameCount>>) -> usize {
    let mut net = 0;
    for contents in all.values() {
        if first_star_helper(all, contents) {
            net += 1;
        }
    }

    net
}

fn second_star_helper(
    all: &HashMap<String, Vec<BagNameCount>>,
    bags: &[BagNameCount],
) -> (usize, usize) {
    let mut accum_bags = 0;
    let mut current_bags = 0;

    if bags.is_empty() {
        return (1, 0);
    }

    for bag in bags {
        let (bags_inside, containing_bags) = second_star_helper(all, all.get(&bag.name).unwrap());
        accum_bags += bag.count * bags_inside;
        current_bags += bag.count * containing_bags;
    }

    // include self
    current_bags += 1;

    (accum_bags, current_bags)
}

fn second_star(all: &HashMap<String, Vec<BagNameCount>>) -> usize {
    let contents = all.get("shiny gold").unwrap();

    let (total, remainder) = second_star_helper(all, contents);

    // -1 don't count ourselves
    total + remainder - 1
}

fn parse(input_raw: &str) -> Result<HashMap<String, Vec<BagNameCount>>, Error> {
    let mut parsed = HashMap::new();

    for line in input_raw.split('\n') {
        // input example: "dotted blue bags contain 3 wavy bronze bags, 5 clear tomato bags."

        // remove "bags" and the dot at the end of a line: "dotted blue contain 3 wavy bronze, 5 clear tomato."
        let line = line.replace("bags", "").replace("bag", "").replace('.', "");

        // split by "contain" words
        let parts: Vec<String> = line.split("contain").map(|item| item.into()).collect();

        // bag_name = "dotted blue", bag_contains = "3 wavy bronze, 5 clear tomato"
        let (bag_name, bags_contains): (String, &str) = (parts[0].trim().into(), parts[1].trim());

        // ["3 way bronze", "5 clear tomato"]
        let bags: Vec<String> = bags_contains
            .split(',')
            .map(|item| item.trim().into())
            .collect();

        for bag in bags {
            let items = parsed.entry(bag_name.clone()).or_insert_with(Vec::new);

            if bag != "no other" {
                let mut splitter = bag.splitn(2, ' ');
                // 3
                let sub_count = splitter.next().unwrap().parse::<usize>()?;
                // way bronze
                let sub_name = splitter.next().unwrap().into();

                items.push(BagNameCount {
                    name: sub_name,
                    count: sub_count,
                });
            }
        }
    }

    Ok(parsed)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&input)),
        Ok(second_star(&input)),
    )))
}
