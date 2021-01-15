mod handle_input;

use handle_input::ParsedInput;
use handle_input::read_input_from_file;
use handle_input::parse_input;
use std::collections::VecDeque;

#[allow(dead_code)]
pub fn run() {
    let input: ParsedInput = read_input_from_file();
    let premble_size = 25;
    let part1_value = get_invalid_num(&input, premble_size).unwrap();
    let sequence: VecDeque<usize> = get_continous_block(&input, part1_value, VecDeque::new(), 0, 0).unwrap();
    println!("part 1 {:?}", part1_value);
    let min_of_sequence = sequence.iter().min().unwrap();
    let max_of_sequence = sequence.iter().max().unwrap();
    let part2_value = max_of_sequence + min_of_sequence;
    println!("part 2 {:?}", part2_value);
}

pub fn get_invalid_num(input: &ParsedInput, premble_size: usize) -> Option<usize> {
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

fn get_continous_block(input: &ParsedInput, invalid_num: usize, sequence: VecDeque<usize>, total: i64, idx: usize) -> Option<VecDeque<usize>> {
    let mut s = sequence;
    if let None = input.get(idx) {
        return None;
    }
    let i = input.get(idx).unwrap();
    if total < invalid_num as i64{
        let t = total + *i as i64;
        s.push_back(*i);
        return get_continous_block(input, invalid_num, s, t, idx+1);
    } 
    if total > invalid_num as i64 {
        let p = s.pop_front().unwrap_or_default();
        let t = total - p as i64;
        return get_continous_block(input, invalid_num, s, t, idx);
    }
    if total == invalid_num as i64 {
        return Some(s);
    }
    return None;
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
    fn test_get_invalid_num() {
        let raw_input = RAWINPUT.to_string();
        let input = parse_input(raw_input);
        let value = get_invalid_num(&input, 5);
        assert_eq!(Some(127), value);
    }

    #[test]
    fn test_get_continous_block() {
        let raw_input = RAWINPUT.to_string();
        let input = parse_input(raw_input);
        let invalid_num = get_invalid_num(&input, 5).unwrap();
        let expected_output: VecDeque<usize> = VecDeque::from(vec!(15, 25, 47, 40));
        let value = get_continous_block(&input, invalid_num, VecDeque::new(), 0, 0).unwrap();
        assert_eq!(expected_output, value);
    }
}
