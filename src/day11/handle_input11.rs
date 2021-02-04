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
    pub fn follow_rules_next(&mut self) {
        let empty_vec: Vec<String> = vec!();
        let empty_str = " ".to_string();
        let input = self.0.clone();
        for (i, row) in input.iter().enumerate() {
            for (j, seat) in row.iter().enumerate() {
                let possible_values: [i32;3] = [-1, 0, 1];
                let mut sur_seats = vec!();
                for x in possible_values.iter() {
                    for y in possible_values.iter() {
                        if x==&0 && y==&0 {
                            continue
                        }
                        sur_seats.push(input
                            .get((i as i32 +x) as usize).unwrap_or(&empty_vec)
                            .get((j as i32+y) as usize).unwrap_or(&empty_str)
                        );
                    }
                }
                let sur_count = sur_seats.iter().filter(|x| &x[..]=="#").count();
                if seat=="L" && sur_count==0 {
                    self.0[i][j] = "#".to_string();
                } else if seat=="#" && sur_count>=4 {
                    self.0[i][j] = "L".to_string();
                }
            }
        }
    }
    pub fn follow_rules_end(&mut self) {
        let mut prev = Self::new();
        while self!=&prev {
            prev = self.clone();
            self.follow_rules_next();
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
    const MOCKOUTPUT_FINAL_STATE: &str = "#.#L.L#.##
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
    fn test_input() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let input = Input::from(rawinput);
        println!("{:?}", input);
        assert!(true);
    }

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
        input.follow_rules_next();
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
        input.follow_rules_next();
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
        input.follow_rules_next();
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
        input.follow_rules_next();
        assert_eq!(mock_output, input);
    }

    #[test]
    fn test_follow_rules_continued() {
        let output_final_state = MOCKOUTPUT_FINAL_STATE.to_string(); 
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        let mut prev = Input::new();
        while &input!=&prev {
            prev = input.clone();
            input.follow_rules_next();
        }
        assert_eq!(input, Input::from(output_final_state));
    }

    #[test]
    fn test_follow_rules_end() {
        let output_final_state = MOCKOUTPUT_FINAL_STATE.to_string(); 
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        input.follow_rules_end();
        assert_eq!(input, Input::from(output_final_state));
    }

    #[test]
    fn test_part1_result() {
        let rawinput = RAWINPUT_STATE1.to_string();
        let mut input = Input::from(rawinput);
        input.follow_rules_end();
        let count = input.count_occ_seats();
        assert_eq!(37, count);
    }
}
