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
<<<<<<< HEAD
	let mut file_content: String = String::new();
=======
	let mut file: File_content: String = String::new();
>>>>>>> 84da3640cab7e9c156fa7aa9bc01b57a9bcc4c39

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let tile_grid: Vec<Vec<char>> = file_content
		.lines()
		.map(|line| line
			.chars()
			.collect())
		.collect();

	get_number_of_tiles_enclosed_by_the_loop(&tile_grid).to_string()
}


fn get_number_of_tiles_enclosed_by_the_loop(tile_grid: &Vec<Vec<char>>) -> usize {
	let mut tile_grid_with_no_unused_tiles: Vec<Vec<char>> = replace_unused_tiles(&tile_grid, get_positions_of_loop_pipes(&tile_grid));

	replace_starting_tile(&mut tile_grid_with_no_unused_tiles);

	let mut n_of_tiles_in_loop: usize = 0;

	for i in 0..tile_grid_with_no_unused_tiles.len() {
		let mut is_in_loop: bool = false;
		let mut previous_corner_tile: char = '.';
		for j in 0..tile_grid_with_no_unused_tiles[i].len() {
			match tile_grid_with_no_unused_tiles[i][j] {
				'|' => {
					is_in_loop = !is_in_loop;
					continue;
				}
				'-' => continue,
				_ => {}
			}

			match previous_corner_tile {
				'F' => {
					match tile_grid_with_no_unused_tiles[i][j] {
						'J' => {is_in_loop = !is_in_loop;}
						'7' => {},
						_ => continue
					}
				},
				'L' => {
					match tile_grid_with_no_unused_tiles[i][j] {
						'7' => {is_in_loop = !is_in_loop;}
						'J' => {},
						_ => continue
					}
				}
				_ => {}
			}

			previous_corner_tile = match tile_grid_with_no_unused_tiles[i][j] {
				'F' => 'F',
				'L' => 'L',
				_ => '.'
			};

			if tile_grid_with_no_unused_tiles[i][j] == '.' && is_in_loop {
				n_of_tiles_in_loop += 1;
			}
		}
	}

	n_of_tiles_in_loop
}


fn get_pipe_of_starting_tile(tile_grid: &Vec<Vec<char>>) -> char {
	let starting_position: [usize; 2] = get_starting_position(tile_grid);
	let next_positions = get_next_positions(&tile_grid, &starting_position);

	match next_positions
		.keys()
		.collect::<Vec<&Direction>>()
		.as_slice() {
		i if i.contains(&&Direction::Up) => {
			if i.contains(&&Direction::Right) {return 'L'}
			if i.contains(&&Direction::Down) {return '|'}
			return 'J'
		}
		i if i.contains(&&Direction::Right) => {
			if i.contains(&&Direction::Down) {return 'F'}
			return '-'
		}
		_ => {
			return '7'
		}
	}
}


fn replace_starting_tile(tile_grid: &mut Vec<Vec<char>>) -> &mut Vec<Vec<char>> {
	let starting_position: [usize; 2] = get_starting_position(tile_grid);
	let starting_tile = get_pipe_of_starting_tile(&tile_grid);

	tile_grid[starting_position[0]][starting_position[1]] = starting_tile;

	tile_grid
}


fn replace_unused_tiles(
	tile_grid: &Vec<Vec<char>>, 
	tiles_to_not_replace: Vec<[usize; 2]>
) -> Vec<Vec<char>> {
	let mut new_tile_grid: Vec<Vec<char>> = tile_grid.to_owned();

	for i in 0..tile_grid.len() {
		for j in 0..tile_grid[i].len() {
			if !tiles_to_not_replace.contains(&[i, j]) {
				new_tile_grid[i][j] = '.';
			}
		}
	}

	new_tile_grid
}


fn get_positions_of_loop_pipes(tile_grid: &Vec<Vec<char>>) -> Vec<[usize; 2]> {
	let starting_position: [usize; 2] = get_starting_position(tile_grid);

	for (direction, next_position) in get_next_positions(&tile_grid, &starting_position) {
		let mut current_position = next_position;
		let mut previous_to_current_direction = direction;
		let mut loop_pipes: Vec<[usize; 2]> = Vec::from([next_position]);

		loop {
			loop_pipes.push(current_position);

			let current_tile: char = tile_grid[current_position[0]][current_position[1]];

			match current_tile {
				'.' => {
					break;
				}
				'S' => {
					return loop_pipes;
				}
				_ => {
				}
			}

			previous_to_current_direction = get_next_direction(current_tile, &previous_to_current_direction);

			if previous_to_current_direction == Direction::None {
				break;
			}

			current_position = get_next_position(current_position, &previous_to_current_direction);
		}
	}

	vec![[0, 0]]
}


fn get_next_positions(tile_grid: &Vec<Vec<char>>, starting_position: &[usize; 2]) -> HashMap<Direction, [usize; 2]> {
	let mut next_positions: HashMap<Direction, [usize; 2]> = HashMap::new();

	if starting_position[0] != 0 {
		let direction = Direction::Up;
		let y = starting_position[0] - 1;
		let x = starting_position[1];
		if get_next_direction( tile_grid[y][x], &direction) != Direction::None {
			next_positions.insert(direction, [y, x]);
		}
	}
	if starting_position[1] + 1 < tile_grid[0].len() {
		let direction = Direction::Right;
		let y = starting_position[0];
		let x = starting_position[1] + 1;
		if get_next_direction( tile_grid[y][x], &direction) != Direction::None {
			next_positions.insert(direction, [y, x]);
		}
	}
	if starting_position[0] + 1 < tile_grid.len() {
		let direction = Direction::Down;
		let y = starting_position[0] + 1;
		let x = starting_position[1];
		if get_next_direction( tile_grid[y][x], &direction) != Direction::None {
			next_positions.insert(direction, [y, x]);
		}
	}
	if starting_position[1] != 0 {
		let direction = Direction::Left;
		let y = starting_position[0];
		let x = starting_position[1] - 1;
		if get_next_direction( tile_grid[y][x], &direction) != Direction::None {
			next_positions.insert(direction, [y, x]);
		}
	}

	return next_positions
}


fn get_starting_position(tile_grid: &Vec<Vec<char>>) -> [usize; 2] {
	for (i, tile_row) in tile_grid.iter().enumerate() {
		for (j, tile) in tile_row.iter().enumerate() {
			if *tile == 'S' {
				return [i, j]
			}
		}
	}
	
	[0, 0]
}


fn get_next_direction(current_tile: char, previous_to_current_direction: &Direction) -> Direction {
	match previous_to_current_direction {
		Direction::Up => match current_tile {
			'|' => Direction::Up,
			'-' => Direction::None,
			'L' => Direction::None,
			'J' => Direction::None,
			'7' => Direction::Left,
			'F' => Direction::Right,
			_ => Direction::None
		},
		Direction::Right => match current_tile {
			'|' => Direction::None,
			'-' => Direction::Right,
			'L' => Direction::None,
			'J' => Direction::Up,
			'7' => Direction::Down,
			'F' => Direction::None,
			_ => Direction::None
		},
		Direction::Down => match current_tile {
			'|' => Direction::Down,
			'-' => Direction::None,
			'L' => Direction::Right,
			'J' => Direction::Left,
			'7' => Direction::None,
			'F' => Direction::None,
			_ => Direction::None
		},
		Direction::Left => match current_tile {
			'|' => Direction::None,
			'-' => Direction::Left,
			'L' => Direction::Up,
			'J' => Direction::None,
			'7' => Direction::None,
			'F' => Direction::Down,
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
		_ => [0, 0]
	}
}