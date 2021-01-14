use std::fs;

pub type ParsedInput<T, F> = Vec<(T, F)>;

pub fn read_input_from_file() -> String {
    const FILEPATH: &str = "input/day8.input";
    let input = fs::read_to_string(FILEPATH).unwrap().to_string();
    return input;
}

pub fn parse_input(raw_input: String) -> ParsedInput<String, i32> {
    raw_input
        .split("\n")
        .filter_map(|inst_set| {
            let idx_of_whitespace = inst_set.find(' ');
            match idx_of_whitespace {
                Some(idx) => {
                    let inst: String = inst_set[..idx].to_string();
                    let value = inst_set[idx..].trim().parse::<i32>().unwrap();
                    return Some((inst, value));
                },
                None => return None
            }
        }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let mock_raw_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6".to_string();
        let parsed_input = parse_input(mock_raw_input);
        let expected_output = vec![
            ("nop".to_string(), 0),
            ("acc".to_string(), 1),
            ("jmp".to_string(), 4),
            ("acc".to_string(), 3),
            ("jmp".to_string(), -3),
            ("acc".to_string(), -99),
            ("acc".to_string(), 1),
            ("jmp".to_string(), -4),
            ("acc".to_string(), 6)
        ];
        println!("{:?}", parsed_input);
        assert_eq!(expected_output, parsed_input);
    }
}

