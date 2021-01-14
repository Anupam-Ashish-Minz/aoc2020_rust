use std::fs;
use std::str::Chars;

#[allow(dead_code)]
pub fn run() {
    let input = read_input_from_file();
    let y: Vec<usize> = input
        .iter()
        .map(|x| -> usize {
            return calc_seatid_from_string(x);
        })
        .collect();
    //let mut max = 0;
    //for i in y {
    //    if i > max {
    //        max = i;
    //    }
    //}
    //println!("The max value is {}", max);
    let x: usize = find_my_seat(y);
    println!("My seat is {}", x);
}

fn read_input_from_file() -> Vec<String> {
    const FILEPATH: &str = "input/day5.input";
    let input: Vec<String> = fs::read_to_string(FILEPATH)
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect();
    input
}

fn calc_v_seat(start: usize, end: usize, mut ch: Chars) -> usize {
    let c = ch.next().unwrap_or_default();
    if c == 'F' {
        return calc_v_seat(start, (end+start)/2, ch);
    } else if c == 'B' {
        return calc_v_seat((end+start)/2+1, end, ch);
    } else if c.is_alphabetic() {
        return calc_v_seat(start, end, ch);
    }
    return start;
}

fn calc_h_seat(start: usize, end: usize, mut ch: Chars) -> usize {
    let c = ch.next().unwrap_or_default();
    if c == 'L' {
        return calc_h_seat(start, (end+start)/2, ch);
    } else if c == 'R' {
        return calc_h_seat((end+start)/2+1, end, ch);
    } else if c.is_alphabetic() {
        return calc_h_seat(start, end, ch);
    }
    return start;
}

fn calc_seatid_from_string(s: &String) -> usize {
    let ch = s.chars();
    let (rmin, rmax) = (0, 127);
    let (cmin, cmax) = (0, 7);
    let row = calc_v_seat(rmin, rmax, ch.clone());
    let col = calc_h_seat(cmin, cmax, ch.clone());
    return row * 8 + col;
}

// part 2
fn find_my_seat(mut input: Vec<usize>) -> usize {
    input.sort();
    let len = input.len()-1;
    let mut x = 0;
    for i in 0..len {
        if input[i+1] - input[i] > 1 {
            x = input[i]+1;
        }
    }
    return x;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_calc_v_seat() {
        let s1 = "FBFBBFFRLR".to_string();
        let ch = s1.chars();
        assert_eq!(44, calc_v_seat(0, 127, ch));
    }

    #[test]
    fn check_calc_h_seat() {
        let s1 = "FBFBBFFRLR".to_string();
        let ch = s1.chars();
        assert_eq!(5, calc_h_seat(0, 7, ch));
    }

    #[test]
    fn check_calc_seatid_from_string() {
        let (s1, r1) = (&"FBFBBFFRLR".to_string(), 357);
        let (s2, r2) = (&"BFFFBBFRRR".to_string(), 567);
        let (s3, r3) = (&"FFFBBBFRRR".to_string(), 119);
        let (s4, r4) = (&"BBFFBBFRLL".to_string(), 820);

        assert_eq!(r1, calc_seatid_from_string(s1));
        assert_eq!(r2, calc_seatid_from_string(s2));
        assert_eq!(r3, calc_seatid_from_string(s3));
        assert_eq!(r4, calc_seatid_from_string(s4));
    }
}
