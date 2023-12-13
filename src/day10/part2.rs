pub fn main(testing: bool) -> String {
	let file_content: &str = if testing {
		include_str!("test1.txt")
	}
	else {
		include_str!("input.txt")
	};

	let tile_grid: Vec<Vec<&str>> = file_content
		.lines()
		.map(|line| line
            .split_whitespace()
            .collect())
		.collect();

	get_farthest_number_of_steps_from_animal(&tile_grid).to_string()
}


fn get_farthest_number_of_steps_from_animal(tile_grid: &Vec<Vec<&str>>) -> u32 {
	let position: [usize; 2] = get_starting_position(tile_grid);

	println!("{:?}", position);

	5
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