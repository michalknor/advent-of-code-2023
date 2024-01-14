use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let garden_map: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| 
            line.chars().collect())
        .collect();

    solve(&garden_map).to_string()
}


fn solve(garden_map: &Vec<Vec<char>>) -> usize {
    let max_coords: (usize, usize) = (garden_map.len() - 1, garden_map[0].len() - 1);

    let mut stack: Vec<((usize, usize), u8)> = vec![(get_starting_position(&garden_map), 0)];
    let mut visited_tiles: Vec<Vec<u8>> = vec![vec![u8::MAX; max_coords.1 + 1]; max_coords.0 + 1];

    let mut number_of_visited_tiles: usize = 0;

    while !stack.is_empty() {
        let (current_position, current_steps) = stack.pop().unwrap();

        if visited_tiles[current_position.0][current_position.1] <= current_steps || garden_map[current_position.0][current_position.1] == '#' || current_steps > 64 {
            continue;
        }
        
        if visited_tiles[current_position.0][current_position.1] == u8::MAX && current_steps % 2 == 0 {
            number_of_visited_tiles += 1;
        }

        visited_tiles[current_position.0][current_position.1] = current_steps;

        if current_position.0 != 0 {
            stack.push(((current_position.0 - 1, current_position.1), current_steps + 1));
        }
        if current_position.0 < max_coords.0 {
            stack.push(((current_position.0 + 1, current_position.1), current_steps + 1));
        }
        if current_position.1 != 0 {
            stack.push(((current_position.0, current_position.1 - 1), current_steps + 1));
        }
        if current_position.1 < max_coords.1 {
            stack.push(((current_position.0, current_position.1 + 1), current_steps + 1));
        }
    }
    
    number_of_visited_tiles
}


fn get_starting_position(garden_map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..garden_map.len() {
        for j in 0..garden_map[0].len() {
            if garden_map[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    (0, 0)
}