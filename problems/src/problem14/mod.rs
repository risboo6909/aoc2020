use std::collections::HashMap;

use failure::{format_err, Error};
use utils::{result, RetTypes};

const LEAST_SIGNIFICANT_ZERO: usize = (1 << 35) - 2;

#[derive(Debug, PartialEq)]
enum MaskBit {
    One,
    Zero,
    None,
}

#[derive(Debug)]
enum Instr {
    Write(usize, usize),
    Mask(Vec<MaskBit>),
}

fn parse(input_raw: &str) -> Result<Vec<Instr>, Error> {
    let mut prog = Vec::new();

    for line in input_raw.lines() {
        let tmp = line.replace(' ', "");
        if tmp.starts_with("mask") {
            let mut splitter = tmp.split('=');
            splitter.next();
            let bitmask: Result<Vec<MaskBit>, Error> = splitter
                .next()
                .ok_or_else(|| format_err!("error parsing bitmask"))?
                .chars()
                .map(|s| match s {
                    '0' => Ok(MaskBit::Zero),
                    '1' => Ok(MaskBit::One),
                    'X' => Ok(MaskBit::None),
                    _ => Err(format_err!("unknown symbol")),
                })
                .rev()
                .collect();
            prog.push(Instr::Mask(bitmask?));
        } else if tmp.starts_with("mem") {
            let mut splitter = tmp.split('=');
            let addr = splitter
                .next()
                .unwrap()
                .replace('[', "")
                .replace(']', "")
                .replace("mem", "")
                .parse::<usize>()?;
            let value = splitter.next().unwrap().parse::<usize>()?;
            prog.push(Instr::Write(addr, value))
        } else {
            return Err(format_err!("unexpected command"));
        }
    }

    Ok(prog)
}

fn first_star(prog: &[Instr]) -> usize {
    let mut cur_bitmask = None;
    let mut mem: HashMap<usize, usize> = HashMap::new();

    for instr in prog {
        match instr {
            Instr::Mask(bitmask) => {
                cur_bitmask = Some(bitmask);
            }
            Instr::Write(addr, mut value) => {
                // apply mask
                for (pos, mask_bit) in cur_bitmask.unwrap().iter().enumerate() {
                    value = match mask_bit {
                        MaskBit::None => value,
                        MaskBit::One => value | (1 << pos),
                        MaskBit::Zero => {
                            value & ((LEAST_SIGNIFICANT_ZERO << pos) | ((1 << pos) - 1))
                        }
                    };
                }
                mem.insert(*addr, value);
            }
        }
    }

    mem.values().sum()
}

fn gen_floating_addrs(addr: usize, mask: &[MaskBit], bit_no: usize) -> Vec<usize> {
    if bit_no >= mask.len() {
        return vec![addr];
    }

    let mut res = vec![];

    if mask[bit_no] == MaskBit::None {
        res.extend(gen_floating_addrs(addr ^ (1 << bit_no), mask, bit_no + 1));
    };

    res.extend(gen_floating_addrs(addr, mask, bit_no + 1));

    res
}

fn second_star(prog: &[Instr]) -> usize {
    let mut cur_bitmask = None;
    let mut mem: HashMap<usize, usize> = HashMap::new();

    for instr in prog {
        match instr {
            Instr::Mask(bitmask) => {
                cur_bitmask = Some(bitmask);
            }
            Instr::Write(mut addr, value) => {
                // apply deterministic bit rules
                for (pos, mask_bit) in cur_bitmask.unwrap().iter().enumerate() {
                    addr = if *mask_bit == MaskBit::One {
                        addr | (1 << pos)
                    } else {
                        addr
                    };
                }

                // apply floating bits rules
                let addrs = gen_floating_addrs(addr, &cur_bitmask.unwrap(), 0);
                for addr in addrs {
                    mem.insert(addr, *value);
                }
            }
        }
    }

    mem.values().sum()
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let prog = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&prog)),
        Ok(second_star(&prog)),
    )))
}
