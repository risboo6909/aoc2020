pub struct GroupByEmptyLine {
    data: Vec<Option<String>>,
    line_idx: usize,
}

impl GroupByEmptyLine {
    pub fn new<T: AsRef<str>>(input: T) -> Self {
        GroupByEmptyLine {
            data: input
                .as_ref()
                .split('\n')
                .map(&|line: &str| {
                    if line.is_empty() {
                        None
                    } else {
                        Some(line.to_owned())
                    }
                })
                .collect(),
            line_idx: 0,
        }
    }
}

impl Iterator for GroupByEmptyLine {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
        if self.line_idx >= self.data.len() {
            return None;
        }

        let mut group = vec![];

        // read until the end of a text block or until the end of a file
        for line in self.data.iter().skip(self.line_idx) {
            self.line_idx += 1;
            match line {
                Some(text) => group.push(text.clone()),
                None => break,
            }
        }

        Some(group)
    }
}

#[cfg(test)]
mod tests {
    use super::GroupByEmptyLine;
    use itertools::assert_equal;

    #[test]
    fn test_1() {
        let input = concat!("abc\n", "def\n", "ghi\n", "\n", "xyz\n", "\n", "ijk\n", "lmn\n",);

        let mut res = vec![];

        for block in GroupByEmptyLine::new(input) {
            res.push(block.clone());
        }

        assert_equal(&res[0], vec!["abc", "def", "ghi"]);
        assert_equal(&res[1], vec!["xyz"]);
        assert_equal(&res[2], vec!["ijk", "lmn"]);
    }

    #[test]
    fn test_2() {
        let input = concat!("abc",);

        let mut res = vec![];

        for block in GroupByEmptyLine::new(input) {
            res.push(block.clone());
        }

        assert_equal(&res[0], vec!["abc"]);
    }
}
