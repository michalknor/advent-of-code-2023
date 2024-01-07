use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
	Down,
    Left,
	None
}


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file: File_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let tile_grid: Vec<Vec<&str>> = file_content
		.lines()
		.map(|line| line
			.split("")
			.collect())
		.collect();

	get_farthest_number_of_steps_from_animal(&tile_grid).to_string()
}


fn get_farthest_number_of_steps_from_animal(tile_grid: &Vec<Vec<&str>>) -> u32 {
	let starting_position: [usize; 2] = get_starting_position(tile_grid);

	let mut next_positions: HashMap<Direction, [usize; 2]> = HashMap::new();

	if starting_position[0] != 0 {
		next_positions.insert(Direction::Up, [starting_position[0] - 1, starting_position[1]]);
	}
	if starting_position[1] + 1 < tile_grid[0].len() {
		next_positions.insert(Direction::Right, [starting_position[0], starting_position[1] + 1]);
	}
	if starting_position[0] + 1 < tile_grid.len() {
		next_positions.insert(Direction::Down, [starting_position[0] + 1, starting_position[1]]);
	}
	if starting_position[1] != 0 {
		next_positions.insert(Direction::Left, [starting_position[0], starting_position[1] - 1]);
	}

	for (direction, next_position) in next_positions {
		let mut current_position = next_position;

		let mut previous_to_current_direction = direction;

		let mut steps = 1;

		loop {
			let current_tile: &str = tile_grid[current_position[0]][current_position[1]];

			match current_tile {
				"." => {
					break;
				}
				"S" => {
					return steps / 2;
				}
				_ => {
				}
			}

			previous_to_current_direction = get_next_direction(current_tile, &previous_to_current_direction);

			if previous_to_current_direction == Direction::None {
				break;
			}

			current_position = get_next_position(current_position, &previous_to_current_direction);
			
			steps += 1;
		}
	}

	0
}


fn get_starting_position(tile_grid: &Vec<Vec<&str>>) -> [usize; 2] {
	for (i, tile_row) in tile_grid.iter().enumerate() {
		for (j, tile) in tile_row.iter().enumerate() {
			if *tile == "S" {
				return [i, j]
			}
		}
	}
	
	return [0, 0]
}


fn get_next_direction(current_tile: &str, previous_to_current_direction: &Direction) -> Direction {
	match previous_to_current_direction {
		Direction::Up => match current_tile {
			"|" => Direction::Up,
			"-" => Direction::None,
			"L" => Direction::None,
			"J" => Direction::None,
			"7" => Direction::Left,
			"F" => Direction::Right,
			_ => Direction::None
		},
		Direction::Right => match current_tile {
			"|" => Direction::None,
			"-" => Direction::Right,
			"L" => Direction::None,
			"J" => Direction::Up,
			"7" => Direction::Down,
			"F" => Direction::None,
			_ => Direction::None
		},
		Direction::Down => match current_tile {
			"|" => Direction::Down,
			"-" => Direction::None,
			"L" => Direction::Right,
			"J" => Direction::Left,
			"7" => Direction::None,
			"F" => Direction::None,
			_ => Direction::None
		},
		Direction::Left => match current_tile {
			"|" => Direction::None,
			"-" => Direction::Left,
			"L" => Direction::Up,
			"J" => Direction::None,
			"7" => Direction::None,
			"F" => Direction::Down,
			_ => Direction::None
		},
		_ => Direction::None
	}
}


fn get_next_position(current_position: [usize; 2], next_pipe_direction: &Direction) -> [usize; 2] {
	match next_pipe_direction {
		Direction::Up => [current_position[0] - 1, current_position[1]],
		Direction::Right => [current_position[0], current_position[1] + 1],
		Direction::Down => [current_position[0] + 1, current_position[1]],
		Direction::Left => [current_position[0], current_position[1] - 1],
		_ => return [0, 0]
	}
}