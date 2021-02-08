mod handle_input12;

pub fn run() {
    
}

fn part1() {
    
}

fn part2() {
    
}

fn follow_inst1(input: Vec<(char, i32)>) {
    // direction, east_west_value int, north_south_value int
    let mut ship_state: (char, i32, i32) = ('E', 0, 0);
    for (dir, val) in input {
        if dir == 'F' {
            if ship_state.0 == 'E' { ship_state.1 += val; }
            else if ship_state.0 == 'N' { ship_state.2 += val; }
            else if ship_state.0 == 'W' { ship_state.1 -= val; }
            else if ship_state.0 == 'S' { ship_state.2 -= val; }
        } 
        else if dir == 'E' { ship_state.1 += val; } 
        else if dir == 'N' { ship_state.2 += val; }
        else if dir == 'W' { ship_state.1 -= val; } 
        else if dir == 'S' { ship_state.2 -= val; }
    }
}
