use std::collections::HashMap;
use std::str;

use failure::{format_err, Error};
use utils::{result, RetTypes};

use prefix_tree::PrefixSet;

enum Node {
    Single(Vec<usize>),
    Multi(Vec<usize>, Vec<usize>),
    Term(String),
}

enum State {
    Rules,
    Input,
}

fn first_star(nodes_map: &HashMap<usize, Node>, input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let p_tree = build_trie(&nodes_map, 0);

    input
        .iter()
        .filter(|line| !p_tree.contains(line))
        .cloned()
        .collect::<Vec<Vec<u8>>>()
}

fn clone_from_node(nodes_map: &HashMap<usize, Node>, node_id: usize) -> HashMap<usize, Node> {
    let mut res = HashMap::new();

    struct Ctx<'a> {
        nodes_map: &'a HashMap<usize, Node>,
        res: &'a mut HashMap<usize, Node>,
    }

    fn traverse(ctx: &mut Ctx, node_id: usize) {
        match &ctx.nodes_map[&node_id] {
            Node::Single(ids) => {
                ctx.res.insert(node_id, Node::Single(ids.clone()));
                for id in ids {
                    traverse(ctx, *id);
                }
            }
            Node::Multi(left, right) => {
                ctx.res
                    .insert(node_id, Node::Multi(left.clone(), right.clone()));
                for id in left.iter().chain(right) {
                    traverse(ctx, *id);
                }
            }
            Node::Term(c) => {
                ctx.res.insert(node_id, Node::Term(c.clone()));
            }
        }
    }

    traverse(
        &mut Ctx {
            nodes_map,
            res: &mut res,
        },
        node_id,
    );

    res
}

fn second_star(nodes_map: &HashMap<usize, Node>, input: &[Vec<u8>]) -> usize {
    // replace 8 and 11 rules to their recursive variants:

    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31

    let p_tree_42 = build_trie(&clone_from_node(nodes_map, 42), 42);
    let p_tree_31 = build_trie(&clone_from_node(nodes_map, 31), 31);

    fn match_tail(p_tree_31: &PrefixSet<u8>, line: &[u8], st: usize, depth: usize) -> bool {
        if depth == 0 {
            return st >= line.len();
        }

        for end in st..=line.len() {
            let slice = &line[st..end];
            if p_tree_31.contains(slice) && match_tail(&p_tree_31, line, end, depth - 1) {
                return true;
            }
        }

        false
    }

    // 11: 42 31 | 42 11 31
    fn apply_11(
        p_tree_42: &PrefixSet<u8>,
        p_tree_31: &PrefixSet<u8>,
        line: &[u8],
        st: usize,
        depth: usize,
    ) -> bool {
        if st >= line.len() {
            return false;
        }

        for end in st..line.len() {
            let slice = &line[st..end];

            if p_tree_42.contains(slice) {
                if match_tail(&p_tree_31, line, end, depth) {
                    return true;
                }

                if apply_11(&p_tree_42, &p_tree_31, &line, end, depth + 1) {
                    return true;
                }
            }
        }

        false
    }

    // 8: 42 | 42 8
    fn apply_8(
        p_tree_42: &PrefixSet<u8>,
        p_tree_31: &PrefixSet<u8>,
        line: &[u8],
        st: usize,
    ) -> bool {
        if st >= line.len() {
            return false;
        }

        for end in st..line.len() {
            let slice = &line[st..end];

            // this means we've found one or more 8s rule
            if p_tree_42.contains(slice) {

                // rule 11 must go after rule 8 (cuz rule 0: 8 11)
                if apply_11(&p_tree_42, &p_tree_31, &line, end, 1) {
                    return true;
                }

                if apply_8(&p_tree_42, &p_tree_31, &line, end) {
                    return true;
                }
            }
        }

        false
    }

    let mut net = 0;
    for line in input {
        if apply_8(&p_tree_42, &p_tree_31, &line, 0) {
            net += 1
        }
    }

    net
}

fn gen_prod(xss: &[Vec<u8>], yss: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut res = vec![];
    for xs in xss {
        for ys in yss {
            res.push(xs.iter().copied().chain(ys.iter().copied()).collect());
        }
    }

    res
}

fn gen_multi_prod(inp: &[Vec<Vec<u8>>]) -> Vec<Vec<u8>> {
    let mut res = inp[0].clone();
    let mut idx = 1;

    while idx < inp.len() {
        res = gen_prod(&res, &inp[idx]);
        idx += 1;
    }

    res
}

fn build_trie(nodes_map: &HashMap<usize, Node>, root_id: usize) -> PrefixSet<u8> {
    let mut resolved: HashMap<usize, PrefixSet<_>> = HashMap::new();

    while resolved.len() < nodes_map.len() {
        for (node_idx, node) in nodes_map {
            if resolved.contains_key(node_idx) {
                continue;
            }

            match node {
                Node::Single(indices) => {
                    if indices.iter().all(|idx| resolved.contains_key(idx)) {
                        let mut trie = PrefixSet::new();
                        let mut new_seq: Vec<Vec<Vec<u8>>> = vec![];

                        for idx in indices {
                            new_seq.push(resolved[idx].iter().collect::<Vec<Vec<u8>>>());
                        }

                        for seq in gen_multi_prod(&new_seq) {
                            trie.insert(seq);
                        }

                        resolved.insert(*node_idx, trie);
                    }
                }

                Node::Multi(left_ind, right_ind) => {
                    let left = left_ind.iter().all(|idx| resolved.contains_key(idx));
                    let right = right_ind.iter().all(|idx| resolved.contains_key(idx));

                    if left && right {
                        let mut trie = PrefixSet::new();

                        for indices in &[left_ind, right_ind] {
                            let mut new_seq: Vec<Vec<Vec<u8>>> = vec![];

                            for idx in indices.iter() {
                                new_seq.push(resolved[idx].iter().collect::<Vec<Vec<u8>>>());
                            }

                            for seq in gen_multi_prod(&new_seq) {
                                trie.insert(seq);
                            }
                        }

                        resolved.insert(*node_idx, trie);
                    }
                }

                Node::Term(c) => {
                    let mut trie = PrefixSet::new();
                    trie.insert(c);
                    resolved.insert(*node_idx, trie);
                }
            }
        }
    }

    let mut trie = PrefixSet::new();
    for seq in resolved[&root_id].iter() {
        trie.insert(seq);
    }

    trie
}

type ParseResult = Result<(HashMap<usize, Node>, Vec<Vec<u8>>), Error>;

fn parse(input_raw: &str) -> ParseResult {
    let mut nodes_map: HashMap<usize, Node> = HashMap::new();
    let mut input: Vec<Vec<u8>> = vec![];
    let mut state = State::Rules;

    // collect data into intermediate hashmap representation

    for line in input_raw.lines() {
        if line.trim().is_empty() {
            state = State::Input;
        }

        if let State::Input = state {
            input.push(Vec::from(line.trim().as_bytes()));
            continue;
        }

        let mut splitter = line.split(':');

        let idx = splitter
            .next()
            .ok_or_else(|| format_err!("error parsing line"))?
            .parse::<usize>()?;
        let tail = splitter.next().ok_or_else(|| format_err!("error parsing line"))?;

        if tail.contains('a') {
            nodes_map.insert(idx, Node::Term(String::from("a")));
        } else if tail.contains('b') {
            nodes_map.insert(idx, Node::Term(String::from("b")));
        } else if tail.contains('|') {
            splitter = tail.trim().split('|');
            let mut indices = vec![];
            for chunk in splitter {
                indices.push(
                    chunk
                        .trim()
                        .split_whitespace()
                        .map(|item| item.parse::<usize>())
                        .collect::<Result<Vec<_>, _>>()?,
                );
            }
            nodes_map.insert(idx, Node::Multi(indices[0].clone(), indices[1].clone()));
        } else {
            let indices = tail
                .trim()
                .split_whitespace()
                .map(|item| item.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;
            nodes_map.insert(idx, Node::Single(indices.clone()));
        }
    }

    // build a graph from created map
    Ok((nodes_map, input))
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let (nodes_map, input) = parse(input_raw)?;

    let inputs_left = first_star(&nodes_map, &input);
    let first_start_res = input.len() - inputs_left.len();

    Ok(RetTypes::Usize(result(
        Ok(first_start_res),
        Ok(first_start_res + second_star(&nodes_map, &inputs_left)),
    )))
}
