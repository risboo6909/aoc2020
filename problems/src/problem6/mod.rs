use std::collections::HashSet;
use failure::Error;
use utils::{result, GroupByEmptyLine, ProblemResult, RetTypes};

fn first_star(groups: &[Vec<String>]) -> ProblemResult<usize> {
    let mut net = 0;
    let mut set = HashSet::new();

    for group in groups {
        set.clear();
        for line in group {
            line.chars().for_each(|c| {
                set.insert(c);
            });
        }
        net += set.len();
    }

    Ok(net)
}

fn second_star(groups: &[Vec<String>]) -> ProblemResult<usize> {

    let mut net = 0;

    let mut set = HashSet::new();
    let mut new_set = HashSet::new();       

    for group in groups {

        set.clear();

        for (idx, line) in group.iter().enumerate() { 
        
            new_set.clear();      
        
            line.chars().for_each(|c| {
                if idx == 0 {
                    set.insert(c);
                } else {
                    new_set.insert(c);
                }
            });

            if !new_set.is_empty() {
                set = set.intersection(&new_set).copied().collect::<HashSet<char>>();
            }

        }

        net += set.len();

    }
    
    Ok(net)
}

fn parse(input_raw: &str) -> Vec<Vec<String>> {
    let mut groups = vec![];

    for group in GroupByEmptyLine::new(input_raw) {
        groups.push(group.clone())
    }

    groups
}

pub(crate) fn solve() -> Result<RetTypes, Error> {
    let input_raw = include_str!("./input");
    let groups = parse(input_raw);

    Ok(RetTypes::Usize(result(first_star(&groups), second_star(&groups))))
}

#[cfg(test)]
mod tests {

    use super::{parse, first_star, second_star};

    const INPUT_RAW: &str = 
    concat!(
        "abc\n",
        "\n",
        "a\n",
        "b\n",
        "c\n",
        "\n",
        "ab\n",
        "ac\n",
        "\n",
        "a\n",
        "a\n",
        "a\n",
        "a\n",
        "\n",
        "b\n",
    );

    #[test]
    fn test_first_star() {
        let groups = parse(&INPUT_RAW);
        assert_eq!(first_star(&groups).unwrap(), 11);
    }

    #[test]
    fn test_second_star() {
        let groups = parse(&INPUT_RAW);
        assert_eq!(second_star(&groups).unwrap(), 6);
    }
}