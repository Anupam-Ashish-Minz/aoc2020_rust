use std::fs;

fn read_input_from_file() -> String {
    let path = "../input/day12.input";
    let rawinput = fs::read_to_string(path)
        .expect("failed to read input from file");
    return rawinput;
}

