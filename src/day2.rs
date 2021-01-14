use std::fs;

#[allow(dead_code)]
pub fn run() {
    let filepath = "input/day2.input";
    let input: Vec<String> = read_input_from_file(filepath);
    read_input_from_file(filepath);
    let output: Vec<_> = input.iter().map(|x| parse_input_string_to_tuples(x)).collect();
    //println!("{:?}", output[0]);

    // testing is_valid_password
    assert_eq!(true, is_valid_password_part1(1, 3, "a", "abcde"));
    assert_eq!(false, is_valid_password_part1(1, 3, "b", "cdefg"));
    // testing ends

    let count = count_valid_passwords(&output, &is_valid_password_part1);
    println!("The total number of valid passwords in part 1 are {}", count);

    // testing part2
    assert_eq!(true, is_valid_password_part2(1, 3, "a", "abcde"));
    assert_eq!(false, is_valid_password_part2(1, 3, "b", "cdefg"));
    assert_eq!(false, is_valid_password_part2(2, 9, "c", "ccccccccc"));
    // testing ends

    let count = count_valid_passwords(&output, &is_valid_password_part2);
    println!("The total number of valid passwords in part 2 are {}", count);
}

fn read_input_from_file(filepath: &str) -> Vec<String> {
    let data_from_file = fs::read_to_string(filepath).expect("Failed to red data");
    data_from_file
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect()
}

fn parse_input_string_to_tuples <'a> (input_str: &'a str) -> (i32, i32, &'a str, &'a str) {
    let idx1 = input_str.find("-").expect("Oops some error");
    let idx2 = input_str.find(" ").expect("Oops some error");
    let idx3 = input_str.find(":").expect("Oops some error");

    (
        input_str[0..idx1].parse::<i32>().unwrap(),
        input_str[idx1+1..idx2].parse::<i32>().unwrap(),
        &input_str[idx2+1..idx3],
        &input_str[idx3+1..].trim()
    )
}

fn is_valid_password_part1(n_min: i32, n_max: i32, cc: &str, input_passwd: &str) -> bool {
    let cc_arr: Vec<_> = input_passwd.chars().filter(|x| x == &cc.chars().next().unwrap()).collect();
    let cc_occurance = cc_arr.len() as i32;
    n_max >= cc_occurance && cc_occurance>= n_min
}

fn count_valid_passwords(input: &Vec<(i32, i32, &str, &str)>, is_valid_password: &dyn Fn(i32, i32, &str, &str) -> bool ) -> i32{
    let mut count=0;
    for i in input {
        let (n1, n2, cc, string) = i;
        if is_valid_password(*n1, *n2, cc, string) {
            count+=1;
        }
    }
    return count
}

fn is_valid_password_part2(n1: i32, n2: i32, cc: &str, input_passwd: &str) -> bool {
    let cc_arr: Vec<char> = input_passwd.chars().collect();
    let n1: usize = (n1 - 1) as usize;
    let n2: usize = (n2 - 1) as usize;
    if cc_arr[n1] == cc.chars().next().unwrap() && cc_arr[n2] == cc.chars().next().unwrap() {
        return false
    } else if cc_arr[n1] == cc.chars().next().unwrap() || cc_arr[n2] == cc.chars().next().unwrap() {
        return true
    }
    return false
}
