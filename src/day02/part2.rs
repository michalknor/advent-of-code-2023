use std::fs::File;
use std::io::Read;


fn evaluate_game(modified_line: &str) -> u32 {
	let (mut max_red, mut max_green, mut max_blue): (u32, u32, u32) = (0, 0, 0);

	for game in modified_line.split("; ") {
		for revealed_cube in game.split(", ") {
			let revealed_cube_vec: Vec<&str> = revealed_cube.split(' ').collect();
			let count: u32 = revealed_cube_vec[0].parse::<u32>().unwrap();
			let color: &str = revealed_cube_vec[1];

			match color {
				"red" => {
					if count > max_red {
						max_red = count;
					}
				}
				"green" => {
					if count > max_green {
						max_green = count;
					}
				}
				"blue" => {
					if count > max_blue {
						max_blue = count;
					}
				}
				_ => {
					panic!("Unknown color: {}", color);
				}
			}
		}
	}
	max_red * max_green * max_blue
}

pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();


	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let mut result: u32 = 0;
	let mut id: u32 = 1;
	for line in file_content.lines() {
		let modified_line: &str = line.trim_start_matches(&format!("Game {}: ", id));

		result += evaluate_game(modified_line);
		
		id += 1;
    }

    result.to_string()
}