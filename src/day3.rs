use std::fs;

#[allow(dead_code)]
pub fn run() {
    test_part();
    let input = read_input_from_file();
    //x, y start from the top left corner
    //input [y][x], ptr (y, x), movement (right, down)
    let count = count_num_trees_part1(&input);
    println!("The total number of trees in part 1 are: {}", count);
    let movement_pattern: [(usize, usize); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut prod_count: usize = 1;
    for i in movement_pattern.iter() {
        let count = count_num_trees_part2(&input, *i);
        prod_count *= count;
    }
    println!("The total number of trees in part 1 are: {}", prod_count);
}

fn read_input_from_file<'a>() -> Vec<Vec<char>> {
    const FILEPATH: &str = "input/day3.input";
    let input: String = fs::read_to_string(FILEPATH).expect("failed to read input from file");
    let input: Vec<Vec<char>> = input
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    assert_eq!('#', input[9][11]);
    input
}

fn move_3r1d(ptr: (usize, usize), pattern_size: usize) -> (usize, usize) {
    let (y, x) = ptr;
    ((y+1), (x+3) % pattern_size)
}

// right, down
// 1 1
// 3 1
// 5 1
// 7 1
// 1 2

fn count_num_trees_part1(input: &Vec<Vec<char>>) -> usize {
    let mut ptr: (usize, usize) = (0, 0);
    let distance = input.len();
    let mut count = 0;
    for i in 0..distance {
        let (y, x) = ptr;
        let pattern_size = input[i].len();
        let c = input[y][x];
        if c == '#' { count+=1; }
        ptr = move_3r1d(ptr, pattern_size);
    }
    count
}

fn count_num_trees_part2(input: &Vec<Vec<char>>, movement_pattern: (usize, usize)) -> usize {
    let mut ptr: (usize, usize) = (0, 0);
    let distance = input.len();
    let mut count = 0;
    while ptr.0 < distance {
        let (pty, ptx) = ptr;
        let (mx, my) = movement_pattern;
        let pattern_size = input[pty].len();
        let c = input[pty][ptx];
        if c == '#' { count+=1; }
        ptr = ((pty+my), (ptx+mx) % pattern_size)
    }
    count
}

fn test_part() {
    let mock_input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#".to_string();
    let input: Vec<Vec<char>> = mock_input
        .split("\n")
        .map(|x| x.chars().collect())
        .collect();
    //let count = count_num_trees_part1(input);
    //assert_eq!(7, count);
    let movement_pattern: [(usize, usize); 5] = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    assert_eq!(2, count_num_trees_part2(&input, movement_pattern[0]));
    assert_eq!(7, count_num_trees_part2(&input, movement_pattern[1]));
    assert_eq!(3, count_num_trees_part2(&input, movement_pattern[2]));
    assert_eq!(4, count_num_trees_part2(&input, movement_pattern[3]));
    assert_eq!(2, count_num_trees_part2(&input, movement_pattern[4]));
}

