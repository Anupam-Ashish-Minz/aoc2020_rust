use std::fs;
use std::str::Chars;

pub fn read_input_from_file() {
    let path = "./input/day11.input";
    let input = fs::read_to_string(path).unwrap();
    println!("{:?}", input);
}

#[derive(Debug)]
pub struct Input(Vec<Vec<String>>);

impl Input {
    pub fn new() -> Self { Input(vec!()) }
    pub fn from(input: String) -> Self {
        let input: Vec<&str> = input.trim().split("\n").map(|x| x.trim()).collect();
        let input: Vec<Vec<String>> = input
            .iter()
            .map(|x| -> Vec<String> { 
                x.chars().map(|y| y.to_string()).collect()
            }).collect();
        return Input(input);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAWINPUT_STATE1: &str = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

    #[test]
    fn test_input() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let input = Input::from(rawinput);
        println!("{:?}", input);
        assert!(false);
    }
}
