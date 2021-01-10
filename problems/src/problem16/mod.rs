use std::collections::{HashMap, HashSet};

use failure::{format_err, Error};
use utils::{result, RetTypes};

#[derive(Debug)]
struct Intervals {
    intervals: Vec<(usize, usize)>,
}

impl Intervals {
    fn new() -> Self {
        Self { intervals: vec![] }
    }

    fn add_interval(&mut self, from: usize, to: usize) -> Result<&mut Self, Error> {
        for int in self.intervals.iter() {
            if (from >= int.0 && from <= int.1) || (to >= int.0 && to <= int.1) {
                // intersections are prohibited
                return Err(format_err!("intervals intersection"));
            }
        }

        self.intervals.push((from, to));

        Ok(self)
    }

    fn is_inside_interval(&self, val: usize) -> bool {
        for int in self.intervals.iter() {
            if val >= int.0 && val <= int.1 {
                return true;
            }
        }
        false
    }
}
#[derive(Debug)]
struct Tickets {
    fields_ranges: HashMap<String, Intervals>,
    fields_order: Vec<String>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Tickets {
    fn new() -> Self {
        Self {
            fields_ranges: HashMap::new(),
            fields_order: vec![],
            my_ticket: vec![],
            nearby_tickets: vec![],
        }
    }

    fn is_valid_number(&self, val: usize) -> bool {
        self.fields_ranges
            .values()
            .any(|int| int.is_inside_interval(val))
    }
}

fn first_star(tickets: &Tickets) -> (usize, HashSet<usize>) {
    let mut net: usize = 0;
    let mut to_remove = HashSet::new();

    for (idx, ticket) in tickets.nearby_tickets.iter().enumerate() {
        let mut invalid_ticket = false;

        for val in ticket {
            if !tickets.is_valid_number(*val) {
                net += val;
                invalid_ticket = true;
            }
        }

        if invalid_ticket {
            to_remove.insert(idx);
        }
    }

    (net, to_remove)
}

fn second_star(tickets: &Tickets) -> usize {
    let mut fields_map = HashMap::new();
    let mut value_idx = 0;

    let mut fields_order = tickets.fields_order.clone();

    // for a given field label find appropriate set of values
    while fields_map.len() < tickets.fields_ranges.len() {

        // start all over
        if value_idx >= tickets.fields_ranges.len() {
            value_idx = 0;
        }

        let mut matched = 0;
        
        let mut found_label = String::new();
        let mut remove_idx = 0;

        'outer: for (order_idx, label) in fields_order.iter().enumerate() {

            let field_intervals = tickets.fields_ranges.get(label).unwrap();
            if !field_intervals.is_inside_interval(tickets.my_ticket[value_idx]) {
                continue;
            }

            for ticket in tickets.nearby_tickets.iter() {
                if !field_intervals.is_inside_interval(ticket[value_idx]) {
                    continue 'outer;
                }
            }

            matched += 1;

            found_label = label.clone();
            remove_idx = order_idx;

        }

        // eleminate only fields we are soure about
        if matched == 1 {
            fields_map.insert(found_label, value_idx);
            fields_order.remove(remove_idx);
        }

        value_idx += 1;
    }

    let mut prod = 1;
    for (label, value_idx) in fields_map {
        if label.starts_with("departure") {
            prod *= tickets.my_ticket[value_idx];
        }
    }

    prod
}

enum ParseState {
    FieldsIntervals,
    MyTicket,
    NearbyTickets,
}

fn parse(input_raw: &str) -> Result<Tickets, Error> {
    let mut state = ParseState::FieldsIntervals;
    let mut tickets = Tickets::new();

    for line in input_raw.lines() {
        if line.is_empty() {
            continue;
        }

        match state {
            ParseState::FieldsIntervals => {
                if line.trim() == "your ticket:" {
                    state = ParseState::MyTicket;
                    continue;
                }
                let mut splitter = line.trim().split(':');
                let label = splitter.next().unwrap();
                let values = splitter.next().unwrap();

                for interval in values.replace(' ', "").split("or") {
                    let mut splitter = interval.split('-');
                    let from = splitter.next().unwrap().parse::<usize>()?;
                    let to = splitter.next().unwrap().parse::<usize>()?;

                    tickets
                        .fields_ranges
                        .entry(label.to_owned())
                        .or_insert_with(Intervals::new)
                        .add_interval(from, to)?;
                }

                tickets.fields_order.push(label.to_owned());

            }
            ParseState::MyTicket => {
                if line.trim() == "nearby tickets:" {
                    state = ParseState::NearbyTickets;
                    continue;
                }
                tickets.my_ticket = line
                    .trim()
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
            }
            ParseState::NearbyTickets => {
                tickets.nearby_tickets.push(
                    line.trim()
                        .split(',')
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect(),
                );
            }
        }
    }

    Ok(tickets)
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let mut tickets = parse(input_raw)?;

    let (err_rate, nearby_tickets_to_remove) = first_star(&tickets);
    let mut new_nearby_tickets = vec![];

    // leave only valid tickets
    for (idx, ticket) in tickets.nearby_tickets.iter().enumerate() {
        if !nearby_tickets_to_remove.contains(&idx) {
            new_nearby_tickets.push(ticket.clone());
        }
    }

    tickets.nearby_tickets = new_nearby_tickets;

    Ok(RetTypes::Usize(result(
        Ok(err_rate),
        Ok(second_star(&tickets)),
    )))
}
