use std::io::{BufRead, BufReader};
use std::fs::File;

struct Position {
    x: usize,
    y: usize 
}

fn main() {
    let landscape = read_file("./report");
    let depth_list = search_landscape(landscape.0, landscape.1, landscape.2);
    print_depth_list(&depth_list);
    println!();
    let farthest: i32 = find_farthest(&depth_list);
    println!("{farthest}");
    // print_landscape(landscape.0);
}

fn find_farthest(depth_list: &[Vec<i32>]) -> i32 {

    let mut greatest: i32 = 0;

    for line in depth_list {
        for depth in line {
            if depth > &greatest {
                greatest = *depth;
            } 
        }
    }

    return greatest;

}

fn search_landscape(landscape: Vec<Vec<char>>, depth_vec: Vec<Vec<i32>>, start: Position) -> Vec<Vec<i32>> {
    
    let mut search_list: Vec<Position> = Vec::new();
    let mut visited: Vec<Position> = Vec::new();
    let mut depth_list: Vec<Vec<i32>> = depth_vec;
    let mut step_count = 0;
    search_list.push(start);

    while search_list.len() > 0 {
        let mut search_stage: Vec<Position> = Vec::new();
        while search_list.len() > 0 { 
            let current_position = search_list.pop().unwrap();
            if !pos_in_vec(&visited, &current_position) {
                depth_list[current_position.y][current_position.x] = step_count;
                let visit_list: Vec<Position> = check_neighbors(&landscape, &current_position);
                search_stage.extend(visit_list);
                visited.push(current_position);            
            }
        }
        search_list.extend(search_stage);
        step_count += 1;
    }

    return depth_list;

}

fn check_neighbors(landscape: &[Vec<char>], pos: &Position) -> Vec<Position> {
    
    let x_pos = pos.x;
    let y_pos = pos.y;
    let mut visit_vec: Vec<Position> = Vec::new();
    
    // Check above
    if y_pos != 0 {
        let above_char: char = landscape[y_pos - 1][x_pos];
        if (above_char == '|') || (above_char == 'F') || (above_char == '7') {
            visit_vec.push(Position { x: x_pos, y: y_pos - 1 });
        }
    }
    
    // Check below
    if y_pos != (landscape.len() - 1) {
        let below_char: char = landscape[y_pos + 1][x_pos];
        if (below_char == '|') || (below_char == 'L') || (below_char == 'J') {
            visit_vec.push(Position { x: x_pos, y: y_pos + 1 });
        }
    }
    
    // Check left
    if x_pos != 0 {
        let left_char: char = landscape[y_pos][x_pos - 1];
        if (left_char == '-') || (left_char == 'L') || (left_char == 'F') {
            visit_vec.push(Position { x: x_pos - 1, y: y_pos });
        }
    }

    // Check right
    if x_pos != (landscape[0].len() - 1) {
        let right_char: char = landscape[y_pos][x_pos + 1];
        if (right_char == '-') || (right_char == '7') || (right_char == 'J') {
            visit_vec.push(Position { x: x_pos + 1, y: y_pos });
        }
    }
    
    return visit_vec;

}

fn pos_in_vec(input_vec: &[Position], input_pos: &Position) -> bool {

    for pos in input_vec {
        if (pos.x == input_pos.x) && (pos.y == input_pos.y) {
            return true;
        }
    }

    return false;
}

fn print_landscape(landscape: Vec<Vec<char>>) {

    for line in landscape {
        for char in line {
            print!("{char}");
        }
        println!();
    }

}

fn print_depth_list(depth_list: &[Vec<i32>]) {

    for line in depth_list{
        for depth in line {
            if *depth == -1 {
                print!(".");
            } else {
                print!("{depth}");
            }
        }
        println!();
    }

}

fn read_file(file_path: &str) -> (Vec<Vec<char>>, Vec<Vec<i32>>, Position) {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file."));
    let mut landscape_vec: Vec<Vec<char>> = Vec::new();
    let mut depth_list: Vec<Vec<i32>> = Vec::new();
    let mut pos = Position {
        x: 0,
        y: 0
    };
    let mut y_count = 0;

    for line in reader.lines() {
        let mut char_vec: Vec<char> = Vec::new();
        let mut int_vec: Vec<i32> = Vec::new();
        let mut x_count = 0;
        for char in line.unwrap().chars() {
            char_vec.push(char); 
            int_vec.push(-1);
            if char == 'S' {
                pos = Position {
                    x: x_count,
                    y: y_count
                };
            }
            x_count += 1;
        }
        landscape_vec.push(char_vec);
        depth_list.push(int_vec);
        y_count += 1;
    }

    return (landscape_vec, depth_list, pos);
}
