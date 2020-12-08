use failure::{format_err, Error};
use utils::{result, ParseResult, RetTypes};

const EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Default)]
struct Passport {
    byr: Option<usize>,  // (Birth Year)
    iyr: Option<usize>,  // (Issue Year)
    eyr: Option<usize>,  // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<usize>,  // (Country ID)
}

impl Passport {
    fn is_valid_1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn check_height(&self, lower_bound: usize, upper_bound: usize) -> bool {
        let hgt = self.hgt.as_ref().unwrap();
        match hgt[..hgt.len() - 2].parse::<usize>() {
            Ok(num) => {
                if num < lower_bound || num > upper_bound {
                    return false;
                }
            }
            Err(_) => return false,
        }
        true
    }

    fn is_valid_2(&self) -> bool {
        if !self.is_valid_1() {
            return false;
        }

        if self.byr.unwrap() < 1920 || self.byr.unwrap() > 2002 {
            return false;
        }

        if self.iyr.unwrap() < 2010 || self.iyr.unwrap() > 2020 {
            return false;
        }

        if self.eyr.unwrap() < 2020 || self.eyr.unwrap() > 2030 {
            return false;
        }

        // check hgt

        if self.hgt.as_ref().unwrap().ends_with("cm") {
            if !self.check_height(150, 193) {
                return false;
            }
        } else if self.hgt.as_ref().unwrap().ends_with("in") {
            if !self.check_height(59, 76) {
                return false;
            }
        } else {
            return false;
        }

        // check hcl

        if !self.hcl.as_ref().unwrap().starts_with('#') {
            return false;
        }

        let color = &self.hcl.as_ref().unwrap()[1..];
        if color.len() != 6 {
            return false;
        }

        if color.to_lowercase() != color {
            return false;
        }

        if !color.chars().all(|s| s.is_ascii_alphanumeric()) {
            return false;
        }

        // check ecl
        if !EYE_COLOR.contains(&self.ecl.as_ref().unwrap().as_str()) {
            return false;
        }

        // check pid
        let pid = &self.pid.as_ref().unwrap();
        if pid.len() != 9 {
            return false;
        }
        if !pid.chars().all(char::is_numeric) {
            return false;
        }

        true
    }
}

fn first_star(input: &[Passport]) -> usize {
    input
        .iter()
        .map(|passp| if passp.is_valid_1() { 1 } else { 0 })
        .sum()
}

fn second_star(input: &[Passport]) -> usize {
    input
        .iter()
        .map(|passp| if passp.is_valid_2() { 1 } else { 0 })
        .sum()
}

fn parse_field(field: &str, pasp: &mut Passport) -> ParseResult<()> {
    let parts: Vec<&str> = field.split(':').collect();
    let (name, value) = (parts[0], parts[1]);

    match name {
        "byr" => pasp.byr = Some(value.parse()?),
        "iyr" => pasp.iyr = Some(value.parse()?),
        "eyr" => pasp.eyr = Some(value.parse()?),
        "hgt" => pasp.hgt = Some(String::from(value)),
        "hcl" => pasp.hcl = Some(String::from(value)),
        "ecl" => pasp.ecl = Some(String::from(value)),
        "pid" => pasp.pid = Some(String::from(value)),
        "cid" => pasp.cid = Some(value.parse()?),
        _ => return Err(format_err!("unknown field name '{}'", name)),
    };

    Ok(())
}

fn parse(input_raw: &str) -> Result<Vec<Passport>, Error> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut cur_pasp: Passport = Passport::default();

    for line in input_raw.trim().split('\n') {
        if line.is_empty() {
            passports.push(cur_pasp);
            cur_pasp = Passport::default();
        } else {
            let attrs = line.split_whitespace().collect::<Vec<&str>>();
            for attr in attrs {
                parse_field(attr, &mut cur_pasp)?;
            }
        }
    }

    passports.push(cur_pasp);

    Ok(passports)
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
    use super::{first_star, parse, second_star};

    const INPUT_RAW: &str = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn test_parse() {
        let parsed = parse(INPUT_RAW).unwrap();

        assert_eq!(parsed.len(), 4);

        // some random checks
        assert_eq!(parsed[3].hgt, Some(String::from("59in")));
        assert_eq!(parsed[2].pid, Some(String::from("760753108")));
        assert_eq!(parsed[1].hcl, Some(String::from("#cfa07d")));
        assert_eq!(parsed[0].cid, Some(147));
    }

    #[test]
    fn test_first() {
        let parsed = parse(INPUT_RAW).unwrap();
        assert_eq!(first_star(&parsed), 2);
    }

    #[test]
    fn test_second() {
        let parsed = parse(INPUT_RAW).unwrap();
        assert_eq!(second_star(&parsed), 2);
    }
}
