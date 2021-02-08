mod handle_input11;

use handle_input11::read_input_from_file;
use handle_input11::Input;
use handle_input11::Input2;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    let mut input = Input::from(read_input_from_file());
    input.follow_rules_end(true);
    let count = input.count_occ_seats();
    println!("part1 ans = {}", count);
}

fn part2() {
    let mut input = Input::from(read_input_from_file());  
    input.follow_rules_end(false);
    let count = input.count_occ_seats();
    println!("part2 ans = {}", count);
}

