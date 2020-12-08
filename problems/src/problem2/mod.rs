use failure::Error;
use utils::{result, split_by_lines, RetTypes};

#[derive(Debug)]
struct PasswdItem {
    first_pos: usize,
    second_pos: usize,
    symbol: char,
    passwd: String,
}

impl PasswdItem {
    fn is_valid_1(&self) -> bool {
        let n = self
            .passwd
            .chars()
            .fold(0, |acc, s| if s == self.symbol { acc + 1 } else { acc });
        n >= self.first_pos && n <= self.second_pos
    }

    fn is_valid_2(&self) -> bool {
        let tmp = self.passwd.chars().collect::<Vec<char>>();
        let idx1 = self.first_pos - 1;
        let idx2 = self.second_pos - 1;
        (tmp[idx1] == self.symbol && tmp[idx2] != self.symbol)
            || (tmp[idx1] != self.symbol && tmp[idx2] == self.symbol)
    }
}

fn first_star(input: &[PasswdItem]) -> usize {
    input
        .iter()
        .map(|attempt| if attempt.is_valid_1() { 1 } else { 0 })
        .sum()
}

fn second_star(input: &[PasswdItem]) -> usize {
    input
        .iter()
        .map(|attempt| if attempt.is_valid_2() { 1 } else { 0 })
        .sum()
}

fn parse(input_raw: &str) -> Result<Vec<PasswdItem>, Error> {
    let res: Vec<PasswdItem> = split_by_lines(input_raw, &|line: &str| {
        let tmp = line.trim().replace(':', " ").replace('-', &" ");
        let v = tmp.split_whitespace().collect::<Vec<&str>>();

        let (first_pos, second_pos, symbol, passwd) = (v[0], v[1], v[2], v[3]);

        Ok(PasswdItem {
            first_pos: first_pos.parse::<usize>()?,
            second_pos: second_pos.parse::<usize>()?,
            symbol: symbol.trim().chars().collect::<Vec<char>>()[0],
            passwd: passwd.trim().to_owned(),
        })
    })?;

    Ok(res)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        Ok(first_star(&input)),
        Ok(second_star(&input)),
    )))
}

#[cfg(test)]
mod tests {
    use super::parse;
    #[test]
    fn test_parse() {
        let res = parse(
            r#"1-3 a: abcde
           1-3 b: cdefg
           2-9 c: ccccccccc"#,
        )
        .unwrap();

        assert_eq!(res.len(), 3);

        assert_eq!(res[0].is_valid_1(), true);
        assert_eq!(res[1].is_valid_1(), false);
        assert_eq!(res[2].is_valid_1(), true);

        assert_eq!(res[0].is_valid_2(), true);
        assert_eq!(res[1].is_valid_2(), false);
        assert_eq!(res[2].is_valid_2(), false);
    }
}
