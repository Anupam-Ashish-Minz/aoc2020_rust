use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn run() {
    let input = read_input_from_file();
    let parsed_input = parse_input(input);
    //println!("The number of valid fields are: {}", count);
    let count = count_valid_passports(parsed_input);
    println!("count = {}", count);
}

fn read_input_from_file<'a>() -> Vec<String> {
    const FILEPATH: &str = "input/day4.input";
    let input: Vec<String> = fs::read_to_string(FILEPATH)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();
    input
}

fn parse_input<'a>(input: Vec<String>) -> Vec<Vec<(String, String)>> {
    input
        .iter()
        .map(|x| x
            .replace("\n", " ")
            .split(" ")
            .map(|y| -> (String, String) {
                let mut y = y.split(":");
                let t1 = y.next().unwrap().to_string();
                let t2 = y.next().unwrap().to_string();
                return (t1, t2);
            })
            .collect())
        .collect()
}

fn count_valid_passports(parsed_input: Vec<Vec<(String, String)>>) -> usize {
    let mut count = 0;
    for i in parsed_input {
        let mut field_count = 0;
        for j in i {
            let field_names = [ "ecl", "pid", "eyr", "hcl", "byr", "iyr", "cid", "hgt" ];
            let current_field = field_names.iter().find(|&x| x == &j.0).unwrap();
            if current_field != &"" && current_field != &"cid" {
                let (key, value): (String, String) = j;
                let key = &key[..];
                let value = &value[..];
                match key {
                    "byr" => {
                        let value = value.parse::<i32>().unwrap();
                        if value >= 1920 && value <= 2002 {
                            field_count += 1;
                        }
                    },
                    "iyr" => {
                        let value = value.parse::<i32>().unwrap();
                        if value >= 2010 && value <= 2020 {
                            field_count += 1;
                        }
                    },
                    "eyr" => {
                        let value = value.parse::<i32>().unwrap();
                        if value >= 2020 && value <= 2030 {
                            field_count += 1;
                        }
                    },
                    "hgt" => {
                        let height_pattern = Regex::new(r"\d*(cm|in)").unwrap();
                        if height_pattern.is_match(value) {
                            let unit_pattern = Regex::new(r"(cm|in)").unwrap();
                            let number_pattern = Regex::new(r"\d*").unwrap();
                            let unit: &str = unit_pattern.captures(value).unwrap().get(0).unwrap().as_str();
                            let numbers: i32 = number_pattern.captures(value).unwrap().get(0).unwrap().as_str().trim().parse::<i32>().unwrap();
                            if unit == "cm" && numbers >= 150 && numbers <= 193 {
                                field_count += 1;
                            } else if unit == "in" && numbers >= 59 && numbers <= 76 {
                                field_count += 1;
                            }
                        }
                    },
                    "hcl" => {
                        let hex_pattern = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                        if hex_pattern.is_match(value) && value.len() == 7 {
                            field_count += 1;
                        }
                    },
                    "ecl" => {
                        let eye_pattern = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
                        if eye_pattern.is_match(value) && value.len() == 3 {
                            field_count += 1;
                        }
                    },
                    "pid" => {
                        let pid_pattern = Regex::new(r"^\d{9}$").unwrap();
                        if pid_pattern.is_match(value) {
                            field_count += 1;
                        }
                    },
                    "cid" => {
                        field_count += 1;
                    },
                    _ => {}
                }
            }
        }
        if field_count == 7 || field_count == 8 {
            count+=1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_input() {
        let input = read_input_from_file();
        assert_eq!("byr:1971
iyr:2017 hgt:160cm
eyr:2020 ecl:hzl
pid:157096267", input[0]);
        assert_eq!("cid:322 hgt:163cm
byr:1969 hcl:#a97842 pid:472877556
iyr:2019
ecl:amb eyr:2030", input[7])
    }

    #[test]
    fn check_parsed_input() {
        let input = vec!("abc:304
something:one like:4345
fali:5543".to_string());
        let parsed_input = parse_input(input);
        //let expected_output = vec!(vec!("abc:304", "something:one", "like:4345", "fali:5543"));
        let expected_output = vec!(vec!(
                ("abc".to_string(), "304".to_string()),
                ("something".to_string(), "one".to_string()),
                ("like".to_string(), "4345".to_string()),
                ("fali".to_string(), "5543".to_string()),
        ));
        assert_eq!(expected_output, parsed_input);
    }

    #[test]
    fn check_validate_passports() {
        let raw_mock_input = "hcl:#cfa07d eyr:2029 byr:1937 pid:7912057436
ecl:hzl
cid:192 hgt:68in iyr:2012";

        let mock_input = raw_mock_input
            .trim()
            .split("\n\n")
            .map(|x| x.to_string())
            .collect();

        let parsed_mock_input = parse_input(mock_input);

        let count = count_valid_passports(parsed_mock_input);

        assert_eq!(0, count);
    }
}

