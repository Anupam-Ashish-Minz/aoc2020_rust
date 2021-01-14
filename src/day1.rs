use std::fs;

#[allow(dead_code)]
pub fn run() {
    // take input from file and convert to integer vector
    let filepath = "input/day1.input";
    let input = fs::read_to_string(filepath).expect("Failed to read input");
    let input: Vec<i32> = input
        .trim()
        .split("\n")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    // println!("The input is {:?}", input);
    
    for i in input.clone() {
        for j in input.clone() {
            if i+j == 2020 {
                println!("{}", i*j);
            }
        }
    }
}
