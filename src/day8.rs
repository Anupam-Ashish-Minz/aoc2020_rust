mod handling_input;

use handling_input::read_input_from_file;
use handling_input::parse_input;
use handling_input::ParsedInput;

#[allow(dead_code)]
pub fn run() {
    let input = read_input_from_file();
    let input: ParsedInput<String, i32> = parse_input(input);
    let junction_node = get_junction_node(&input, 0, vec!()).unwrap();
    let loop_chain = get_loop_chain(&input, junction_node as usize, vec!()).unwrap();
    let error_idx = find_error_idx(&input, loop_chain).unwrap();
    let input = modify_input(&input, error_idx);
    let ans = process_inst(&input, 0, vec!(), 0);
    println!("part 2 ans is {:?}", ans);
}

fn process_inst(inst_set: &ParsedInput<String, i32>, current_inst_idx: usize, visited_inst: Vec<usize>, acc: i32) -> Option<i32> {
    let mut result: Option<i32> = None;
    if visited_inst.iter().find(|x| x==&&current_inst_idx).is_none() {
        if let None = inst_set.get(current_inst_idx) {
            return Some(acc);
        }
        let (inst, value) = inst_set.get(current_inst_idx).unwrap();
        let mut visited_inst = visited_inst;
        visited_inst.push(current_inst_idx);
        if inst == "acc" {
            let acc = acc + value;
            result = process_inst(inst_set, current_inst_idx+1, visited_inst, acc);
        } else if inst == "jmp" {
            let current_inst_idx = (current_inst_idx as i32 + *value) as usize;
            result = process_inst(inst_set, current_inst_idx, visited_inst, acc);
        } else if inst == "nop" {
            result = process_inst(inst_set, current_inst_idx+1, visited_inst, acc);
        }
    } else {
        result = Some(acc);
    }
    return result;
}

fn get_junction_node(inst_set: &ParsedInput<String, i32>, current_inst_idx: usize, visited_inst: Vec<usize>) -> Option<i32> {
    let mut result: Option<i32> = None;
    if visited_inst.iter().find(|x| x==&&current_inst_idx).is_none() {
        let (inst, value) = inst_set.get(current_inst_idx).unwrap();
        let mut visited_inst = visited_inst;
        visited_inst.push(current_inst_idx);
        if inst == "acc" {
            result = get_junction_node(inst_set, current_inst_idx+1, visited_inst);
        } else if inst == "jmp" {
            let current_inst_idx = (current_inst_idx as i32 + *value) as usize;
            result = get_junction_node(inst_set, current_inst_idx, visited_inst);
        } else if inst == "nop" {
            result = get_junction_node(inst_set, current_inst_idx+1, visited_inst);
        }
    } else {
        return Some(current_inst_idx as i32);
    }
    return result;
}

fn get_loop_chain(inst_set: &ParsedInput<String, i32>, idx: usize, visited_nodes: Vec<usize>) -> Option<Vec<usize>> {
    let mut result: Option<Vec<usize>> = None;
    let mut visited_nodes = visited_nodes; 
    if visited_nodes.iter().find(|x| x==&&idx).is_none() {
        visited_nodes.push(idx);
        let (inst, value) = inst_set.get(idx).unwrap();
        if inst == "acc" || inst == "nop" {
            result = get_loop_chain(inst_set, idx+1, visited_nodes);
        } else if inst == "jmp" {
            let next_inst_idx = (idx as i32+value) as usize;
            result = get_loop_chain(inst_set, next_inst_idx, visited_nodes);
        }
    } else {
        return Some(visited_nodes);
    }
    return result;
}

fn find_error_idx(inst_set: &ParsedInput<String, i32>, loop_chain:Vec<usize>) -> Option<usize> {
    let mut result = false;
    for idx in loop_chain {
        let (inst, value) = inst_set.get(idx).unwrap();
        if inst == "jmp" {
            result = traverse_graph(inst_set, idx+1, vec!(idx));
        } else if inst == "nop" {
            let next_idx = (idx as i32+value)as usize;
            result = traverse_graph(inst_set, next_idx, vec!(idx));
        }
        if result == true {
            return Some(idx);
        }
    }
    return None;
}

fn traverse_graph(inst_set: &ParsedInput<String, i32>, idx: usize, visited_nodes: Vec<usize>) -> bool {
    let mut result = false;
    let mut visited_nodes = visited_nodes; 
    if visited_nodes.iter().find(|x| x==&&idx).is_none() {
        if let None = inst_set.get(idx) {
            return true;
        }
        let (inst, value) = inst_set.get(idx).unwrap();
        visited_nodes.push(idx);
        if inst == "acc" {
            result = traverse_graph(inst_set, idx+1, visited_nodes);
        } else if inst == "jmp" {
            let next_idx = (idx as i32+value) as usize;
            result = traverse_graph(inst_set, next_idx, visited_nodes);
        } else if inst == "nop" {
            result = traverse_graph(inst_set, idx+1, visited_nodes);
        }
    } else {
        result = false;
    }
    return result;
}

fn modify_input(inst_set: &ParsedInput<String, i32>, idx: usize) -> ParsedInput<String, i32> {
    let mut inst_set = inst_set.clone();
    let (mut inst, value) = inst_set.remove(idx);
    if inst == "jmp" {
        inst = "nop".to_string();
    } else if inst == "nop" {
        inst = "jmp".to_string();
    }
    inst_set.insert(idx, (inst, value));
    return inst_set;
}

#[cfg(test)]
mod test {
    use super::*;

    static RAW_INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_process_inst() {
        let  raw_input = RAW_INPUT.to_string();
        let input: ParsedInput<String, i32> = parse_input(raw_input);
        let result = process_inst(&input, 0, vec!(), 0);
        assert_eq!(Some(5 as i32), result);
    }

    #[test]
    fn test_get_junction_node() {
        let raw_input = RAW_INPUT.to_string();
        let input: ParsedInput<String, i32> = parse_input(raw_input);
        let actual_output = get_junction_node(&input, 0, vec!());
        let expected_output = Some(1);
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_get_loop_chain() {
        let raw_input = RAW_INPUT.to_string();
        let input: ParsedInput<String, i32> = parse_input(raw_input);
        let junction_node = get_junction_node(&input, 0, vec!()).unwrap();
        let actual_output = get_loop_chain(&input, junction_node as usize, vec!());
        let expected_output = Some(vec!(1, 2, 6, 7, 3, 4));
        assert_eq!(expected_output, actual_output);
    }

    #[test]
    fn test_find_error_idx() {
        let raw_input = RAW_INPUT.to_string();
        let input: ParsedInput<String, i32> = parse_input(raw_input);
        let junction_node = get_junction_node(&input, 0, vec!()).unwrap();
        let loop_chain = get_loop_chain(&input, junction_node as usize, vec!()).unwrap();
        let error_idx = find_error_idx(&input, loop_chain);
        assert_eq!(error_idx, Some(7));
    }

    #[test]
    fn test_modify_input() {
        let raw_input = RAW_INPUT.to_string();
        let input: ParsedInput<String, i32> = parse_input(raw_input);
        let junction_node = get_junction_node(&input, 0, vec!()).unwrap();
        let loop_chain = get_loop_chain(&input, junction_node as usize, vec!()).unwrap();
        let error_idx = find_error_idx(&input, loop_chain).unwrap();
        let output = modify_input(&input, error_idx);
        let test_output = vec!(
            ("nop".to_string(), 0),
            ("acc".to_string(), 1),
            ("jmp".to_string(), 4),
            ("acc".to_string(), 3),
            ("jmp".to_string(), -3),
            ("acc".to_string(), -99),
            ("acc".to_string(), 1),
            ("nop".to_string(), -4),
            ("acc".to_string(), 6),
        );
        assert_eq!(test_output, output);
    }

    #[test]
    fn test_ans_part2() {
        let raw_input = RAW_INPUT.to_string();
        let input: ParsedInput<String, i32> = parse_input(raw_input);
        let junction_node = get_junction_node(&input, 0, vec!()).unwrap();
        let loop_chain = get_loop_chain(&input, junction_node as usize, vec!()).unwrap();
        let error_idx = find_error_idx(&input, loop_chain).unwrap();
        let input = modify_input(&input, error_idx);
        let ans = process_inst(&input, 0, vec!(), 0).unwrap();
        assert_eq!(ans, 8);
    }
}


