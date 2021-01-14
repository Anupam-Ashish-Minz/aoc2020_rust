use std::fs;

pub type ParsedInput = Vec<usize>;

pub fn read_input_from_file() -> ParsedInput {
    const FILEPATH: &str = "input/day9.input";
    let input: String = fs::read_to_string(FILEPATH).unwrap();
    return parse_input(input);
}

pub fn parse_input(input: String) -> ParsedInput {
    input
        .trim()
        .split("\n")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}
