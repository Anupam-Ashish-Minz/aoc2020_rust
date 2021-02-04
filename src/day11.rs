mod handle_input11;

use handle_input11::read_input_from_file;
use handle_input11::Input;

pub fn run() {
    part1();
}

fn part1() {
    let mut input = Input::from(read_input_from_file());
    input.follow_rules_end();
    let count = input.count_occ_seats();
    println!("part1 ans = {}", count);
}
