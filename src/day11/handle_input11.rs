use std::fs;

pub fn read_input_from_file() -> String {
    let path = "./input/day11.input";
    let input = fs::read_to_string(path).unwrap();
    return input;
}

#[derive(Debug, Eq, Clone)]
pub struct Input(Vec<Vec<String>>);

#[allow(dead_code)]
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

#[allow(dead_code)]
impl Input {
    fn get_sur_seats_1(&self, i: usize, j: usize, _: usize, _: usize) -> Vec<String> {
        let empty_vec: Vec<String> = vec!();
        let empty_str = " ".to_string();
        let possible_values: [i32;3] = [-1, 0, 1];
        let mut sur_seats = vec!();
        for x in possible_values.iter() {
            for y in possible_values.iter() {
                if x==&0 && y==&0 {
                    continue
                }
                sur_seats.push(self.0
                    .get((i as i32+x) as usize).unwrap_or(&empty_vec)
                    .get((j as i32+y) as usize).unwrap_or(&empty_str).clone()
                );
            }
        }
        return sur_seats;
    }
    fn get_sur_seats_2(&self, i: usize, j: usize, i_max: usize, j_max: usize) -> Vec<String> {
        let empty_vec: Vec<String> = vec!();
        let empty_str = " ".to_string();
        let possible_values: [i32;3] = [-1, 0, 1];
        let mut sur_seats = vec!();
        for x in possible_values.iter() {
            for y in possible_values.iter() {
                if x==&0 && y==&0 {
                    continue
                }
                for a in 1..i_max+j_max {
                    let new_i = i as i32+x*a as i32;
                    let new_j = j as i32+y*a as i32;
                    if new_i > i_max as i32 || new_i < 0 || new_j > j_max as i32 || new_j < 0 {
                        break
                    }
                    let seat = self.0
                        .get(new_i as usize).unwrap_or(&empty_vec)
                        .get(new_j as usize).unwrap_or(&empty_str).clone();
                    if seat=="#" || seat=="L" { 
                        sur_seats.push(seat);
                        break 
                    }
                    sur_seats.push(seat);
                }
            }
        }
        return sur_seats;
    }
    pub fn follow_rules_next(&mut self, is_method_1: bool) {
        let mut input = self.0.clone();
        let input2 = self.0.clone();
        for (i, row) in input2.iter().enumerate() {
            for (j, seat) in row.iter().enumerate() {
                let sur_seats;
                let sur_count;
                if is_method_1 {
                    sur_seats = self.get_sur_seats_1(i, j, 0, 0);
                    sur_count = sur_seats.iter().filter(|x| &x[..]=="#").count();
                    if seat=="L" && sur_count==0 {
                        input[i][j] = "#".to_string();
                    } else if seat=="#" && sur_count>=4 {
                        input[i][j] = "L".to_string();
                    }
                } else {
                    let i_max = self.0.len();
                    let j_max = self.0[0].len();
                    sur_seats = self.get_sur_seats_2(i, j, i_max, j_max);
                    sur_count = sur_seats.iter().filter(|x| &x[..]=="#").count();
                    if seat=="L" && sur_count==0 {
                        input[i][j] = "#".to_string();
                    } else if seat=="#" && sur_count>=5 {
                        input[i][j] = "L".to_string();
                    }
                }
            }
        }
        self.0 = input;
    }
    pub fn follow_rules_end(&mut self, is_method1: bool) {
        let mut prev = Self::new();
        while self!=&prev {
            prev = self.clone();
            self.follow_rules_next(is_method1);
        }
    }
    pub fn count_occ_seats(&self) -> usize {
        let mut count = 0;
        for row in self.0.iter() {
            for seat in row.iter() {
                if seat==&"#" {
                    count+=1;
                }
            }
        }
        return count;
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const RAWINPUT_STATE1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
    const MOCKOUTPUT_FINAL_STATE_PART1: &str = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

    #[test]
    fn test_follow_rules_next() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);

        let raw_mock_output = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##".to_string();
        let mock_output = Input::from(raw_mock_output);
        input.follow_rules_next(true);
        assert_eq!(mock_output, input);

        let raw_mock_output2 = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##".to_string();
        let mock_output = Input::from(raw_mock_output2);
        input.follow_rules_next(true);
        assert_eq!(mock_output, input);

        let raw_mock_output3 = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##".to_string();
        let mock_output = Input::from(raw_mock_output3);
        input.follow_rules_next(true);
        assert_eq!(mock_output, input);

        let raw_mock_output4 = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##".to_string();
        let mock_output = Input::from(raw_mock_output4);
        input.follow_rules_next(true);
        assert_eq!(mock_output, input);
    }

    #[test]
    fn test_follow_rules_continued() {
        let output_final_state = MOCKOUTPUT_FINAL_STATE_PART1.to_string(); 
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        let mut prev = Input::new();
        while &input!=&prev {
            prev = input.clone();
            input.follow_rules_next(true);
        }
        assert_eq!(input, Input::from(output_final_state));
    }

    #[test]
    fn test_follow_rules_end() {
        let output_final_state = MOCKOUTPUT_FINAL_STATE_PART1.to_string(); 
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        input.follow_rules_end(true);
        assert_eq!(input, Input::from(output_final_state));
    }

    #[test]
    fn test_part1_result() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        input.follow_rules_end(true);
        let count = input.count_occ_seats();
        assert_eq!(37, count);
    }

    #[test]
    fn test_follow_rules2() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        let raw_output1 = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL".to_string(); 
        let output = Input::from(raw_output1);
        assert_eq!(input, output);

        input.follow_rules_next(false);

        let raw_output2 = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##".to_string();
        let output = Input::from(raw_output2);
        assert_eq!(input, output);

        input.follow_rules_next(false);

        let raw_output3 = "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#".to_string();
        let output = Input::from(raw_output3);
        assert_eq!(input, output);

        input.follow_rules_next(false);

        let raw_output4 = "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#".to_string();
        let output = Input::from(raw_output4);
        assert_eq!(input, output);

        input.follow_rules_next(false);

        let raw_output5 = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#".to_string();
        let output = Input::from(raw_output5);
        assert_eq!(input, output);

        input.follow_rules_next(false); 

        let raw_output6 = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#".to_string();
        let output = Input::from(raw_output6);
        assert_eq!(input, output);

        input.follow_rules_next(false);

        let raw_output7 = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#".to_string();
        let output = Input::from(raw_output7);
        assert_eq!(input, output);
    }

    #[test]
    fn test_follow_rules2_end() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        let rawoutput = "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#".to_string();
        let output = Input::from(rawoutput);
        input.follow_rules_end(false);
        assert_eq!(input, output);
    }
}


pub struct Input2( Vec<Vec<String>> );

#[allow(dead_code)]
impl Input2 {
    pub fn new() -> Self {
        Input2(vec!())
    }
    pub fn from(value: String) -> Self {
        let input: Vec<Vec<String>> = value.split('\n')
            .map(|x| x.chars()
                .map(|y| y.to_string())
                .collect())
            .collect();
        Input2(input)
    }
}

