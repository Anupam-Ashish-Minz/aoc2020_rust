mod handle_input;

use handle_input::ParsedInput;
use handle_input::read_input_from_file;
use handle_input::parse_input;

#[allow(dead_code)]
pub fn run() {
    let input: ParsedInput = read_input_from_file();
    let premble_size = 25;
    let part1_value = get_invalid_num(input, premble_size);
    println!("part 1 {:?}", part1_value);
}

pub fn get_invalid_num(input: ParsedInput, premble_size: usize) -> Option<usize> {
    let mut value: Option<usize> = None;
    input.iter().enumerate().for_each(|(i, x)| {
        if i < premble_size {return}
        let premble: &[usize] = &input[i-premble_size..i];
        let premble: ParsedInput = premble.iter().map(|x| *x).collect();
        if !is_valid_num(&premble, *x) {
            value = Some(*x);
            return
        }
    });
    return value;
}

fn is_valid_num(premble: &ParsedInput, num: usize) -> bool {
    for i in premble {
        for j in premble {
            if i+j == num {
                return true
            }
        }
    }
    return false;
}


#[cfg(test)]
mod test {
    use super::*;

    const RAWINPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_is_valid_num() {
        let raw_input = RAWINPUT.to_string();
        let input = parse_input(raw_input);
        let value = get_invalid_num(input, 5);
        assert_eq!(Some(127), value);
    }
}
