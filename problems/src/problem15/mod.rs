use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use failure::Error;
use fxhash::{FxHasher, FxBuildHasher};

use utils::{result, RetTypes};

type SeqIndex = usize;

fn find_last_number(seq: &HashMap<usize, Vec<SeqIndex>, BuildHasherDefault<FxHasher>>) -> usize {
    let tmp = seq
        .iter()
        .max_by_key(|(_, indices)| indices.iter().max().unwrap())
        .unwrap();
    *tmp.0
}

fn find_last(seq: &mut HashMap<usize, Vec<SeqIndex>, BuildHasherDefault<FxHasher>>, scan_until: usize) -> usize {
    let mut last_num = find_last_number(seq);

    assert!(scan_until > seq.len());

    for _ in 0..scan_until - seq.len() {
        
        let indices = &seq[&last_num];
        let l = indices.len();

        // there must be at least one index
        let last_idx = *indices.last().unwrap();

        if l == 1 {
            last_num = 0;
        } else {
            // there must be at least 2 indices
            last_num = last_idx - indices[l - 2];
        }

        seq.entry(last_num)
            .or_insert_with(Vec::new)
            .push(last_idx + 1);
    }

    last_num
}

fn first_star(seq: &mut HashMap<usize, Vec<SeqIndex>, BuildHasherDefault<FxHasher>>) -> usize {
    find_last(seq, 2020)
}

fn second_star(seq: &mut HashMap<usize, Vec<SeqIndex>, BuildHasherDefault<FxHasher>>) -> usize {
    find_last(seq, 30000000)
}

fn parse(input_raw: &str) -> Result<HashMap<usize, Vec<SeqIndex>, BuildHasherDefault<FxHasher>>, Error> {
    let mut res = HashMap::with_hasher(FxBuildHasher::default());
    input_raw
        .split(',')
        .enumerate()
        .try_for_each::<_, Result<(), Error>>(|(idx, item)| {
            let n = item.parse::<usize>()?;
            res.entry(n).or_insert_with(Vec::new).push(idx + 1);
            Ok(())
        })?;

    Ok(res)
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let mut start_seq = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&mut start_seq.clone())),
        Ok(second_star(&mut start_seq)),
    )))
}
