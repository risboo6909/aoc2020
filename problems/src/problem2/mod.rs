use failure::Error;
use utils::{result, split_by_lines, ProblemResult, RetTypes};

#[derive(Debug)]
struct PasswdItem {
    first_number: usize,
    second_number: usize,
    symbol: char,
    passwd: String,
}

impl PasswdItem {
    fn is_valid_1(&self) -> bool {
        let n = self
            .passwd
            .chars()
            .fold(0, |acc, s| if s == self.symbol { acc + 1 } else { acc });
        n >= self.first_number && n <= self.second_number
    }

    fn is_valid_2(&self) -> bool {
        let tmp = self.passwd.chars().collect::<Vec<char>>();
        let idx1 = self.first_number - 1;
        let idx2 = self.second_number - 1;
        (tmp[idx1] == self.symbol && tmp[idx2] != self.symbol)
            || (tmp[idx1] != self.symbol && tmp[idx2] == self.symbol)
    }
}

fn first_star(input: &[PasswdItem]) -> ProblemResult<usize> {
    Ok(input
        .iter()
        .map(|attempt| if attempt.is_valid_1() { 1 } else { 0 })
        .sum())
}

fn second_star(input: &[PasswdItem]) -> ProblemResult<usize> {
    Ok(input
        .iter()
        .map(|attempt| if attempt.is_valid_2() { 1 } else { 0 })
        .sum())
}

fn parse(input_raw: &str) -> Result<Vec<PasswdItem>, Error> {
    let res: Vec<PasswdItem> = split_by_lines(input_raw, &|line: &str| {
        // to parse "1-3 a: abcde", first split by ":"
        let parts = line.trim().split(':').collect::<Vec<&str>>();
        let (prefix, suffix) = (parts[0], parts[1]);

        // then split by " " to parse "1-3 a" part
        let parts = prefix.split(' ').collect::<Vec<&str>>();
        let (interval, symbol) = (parts[0], parts[1]);

        // finally, parse "1-3" part
        let parts = interval.split('-').collect::<Vec<&str>>();
        let (min_appear, max_apprar) = (parts[0], parts[1]);

        Ok(PasswdItem {
            first_number: min_appear.parse::<usize>()?,
            second_number: max_apprar.parse::<usize>()?,
            symbol: symbol.trim().chars().collect::<Vec<char>>()[0],
            passwd: suffix.trim().to_owned(),
        })
    })?;

    Ok(res)
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let input = parse(input_raw)?;

    Ok(RetTypes::Usize(result(
        first_star(&input),
        second_star(&input),
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
