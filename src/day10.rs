use handle_input::Input;
use std::collections::VecDeque;

mod handle_input;

pub fn run() {
    let input = Input::read_input_from_file();
    let diff_vec = compute_diff(input);
    let (count1, _count2, count3) = count_diffs(diff_vec);
    println!("day10 part1 {}", count1*count3);
}

fn compute_diff(input: Input) -> Vec<usize> {
    let mut input = input;
    input.sort();
    let mut values: VecDeque<usize> = input.clone();
    let mut diff_result: Vec<usize> = vec!();
    let mut prev = 0;
    while values!=[] {
        let val: usize = values.pop_front().unwrap();
        diff_result.push(val-prev);
        prev = val;
    }
    // push 3 to diff results as 3 is the difference between the build adapter
    // and other adapters
    diff_result.push(3);
    return diff_result;
}

fn count_diffs(diff_vec: Vec<usize>) -> (usize, usize, usize) {
    let mut count1=0;
    let mut count2=0;
    let mut count3=0;
    let prev=0;
    for i in diff_vec {
        if i-prev == 1 {
            count1+=1;
        } else if i-prev == 2 {
            count2+=1;
        } else if i-prev == 3 {
            count3+=1;
        }
    }
    return (count1, count2, count3);
}

#[cfg(test)]
mod test {
    use super::*;

    const RAWINPUT1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const RAWINPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    #[test]
    fn test_count_diffs() {
        let input1 = Input::from(RAWINPUT1);
        let input2 = Input::from(RAWINPUT2);
        let diff_vec1 = compute_diff(input1);
        let diff_vec2 = compute_diff(input2);
        let output1 = count_diffs(diff_vec1);
        let output2 = count_diffs(diff_vec2);
        assert_eq!((7,0,5), output1);
        assert_eq!((22,0,10), output2);
    }
}
