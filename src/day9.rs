mod handle_input;

use handle_input::ParsedInput;
use handle_input::read_input_from_file;
use handle_input::parse_input;

#[allow(dead_code)]
pub fn run() {
    let input: ParsedInput = read_input_from_file();
    let premble = &input[..25];
    let input = &input[25..];
    let premble_size = 25;
    let mut value: Option<usize> = None;
    input.iter().enumerate().for_each(|(i, x)| {
        if i < premble_size {return}
        let premble: &[usize] = &input[i-premble_size..i];
        let premble: ParsedInput = premble.iter().map(|x| *x).collect();
        if !is_valid_num(&premble, *x) {
            println!("{}", x);
            value = Some(*x);
            return
        }
    });
    println!("part 1 {:?}", value);
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
        //let premble = &input[..5];
        //let input = &input[5..];
        //for i in input {
        //    let premble = &input[..5];
        //}
        let mut value: Option<usize> = None;
        let premble_size = 5;
        input.iter().enumerate().for_each(|(i, x)| {
            if i < premble_size {return}
            let premble: &[usize] = &input[i-premble_size..i];
            let premble: ParsedInput = premble.iter().map(|x| *x).collect();
            if !is_valid_num(&premble, *x) {
                value = Some(*x);
            }
        });
        assert_eq!(Some(127), value);
    }
}
