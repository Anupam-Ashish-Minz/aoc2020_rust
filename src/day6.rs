use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn run() {
    let input = read_input_from_file();
    let count = calc_total_sum_part_1(&input);
    println!("part1 = {}", count);
    let count = calc_total_sum_part_2(&input);
    println!("part2 = {}", count);
}

fn read_input_from_file() -> Vec<String> {
    const FILEPATH: &str = "input/day6.input";
    return fs::read_to_string(FILEPATH)
        .unwrap()
        .split("\n\n")
        .map(|x| x.to_string())
        .collect();
}

fn calc_total_sum_part_1(input: &Vec<String>) -> usize {
    let input: Vec<Vec<char>> = input
        .iter()
        .map(|x| x
            .chars()
            .unique()
            .filter(|x| x != &' ' && x != &'\n')
            .collect())
        .collect();
    let mut count = 0;
    for i in input {
        count += i.len(); 
    }
    return count;
}

fn calc_total_sum_part_2(input: &Vec<String>) -> usize {
    let mut count = 0;
    for i in input {
        //let prev_count = count;
        let mut hash_map: HashMap<char, usize> = HashMap::new();
        let entry = i.split("\n");
        let entry_clone = entry.clone();
        entry.for_each(|j| {
            j.chars().for_each(|x| {
                let prev_value: usize = match hash_map.get(&x) {
                    Some(x) => *x,
                    None => 0 as usize,
                };
                hash_map.insert(x, prev_value+1);
            });
        });
        let len: usize = entry_clone.count();
        for j in hash_map.iter() {
            let (_, value) = j;
            if value >= &len {
                count += 1;
            }
        }
        //println!("{:?} {}", hash_map, len);
        //println!("count {}", (count-prev_count));
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_total_sum_part1() {
        let raw_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input: Vec<String> = raw_input
            .split("\n\n")
            .map(|x| x.to_string())
            .collect();

        assert_eq!(11, calc_total_sum_part_1(&input));
    } 

    #[test]
    fn check_calc_total_part_2() {
        let raw_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let input: Vec<String> = raw_input
            .split("\n\n")
            .map(|x| x.to_string())
            .collect();

        let output = calc_total_sum_part_2(&input);
        assert_eq!(6, output); 
    }
}
