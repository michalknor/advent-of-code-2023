use std::fs::File;
use std::io::Read;


const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn evaluate_game(modified_line: &str, game_id: u32) -> u32 {
	for game in modified_line.split("; ") {
		for revealed_cube in game.split(", ") {
			let revealed_cube_vec: Vec<&str> = revealed_cube.split(' ').collect();
			let count: u32 = revealed_cube_vec[0].parse::<u32>().unwrap();
			let color: &str = revealed_cube_vec[1];

			match color {
				"red" => {
					if count > MAX_RED {
						return 0;
					}
				}
				"green" => {
					if count > MAX_GREEN {
						return 0;
					}
				}
				"blue" => {
					if count > MAX_BLUE {
						return 0;
					}
				}
				_ => {
					panic!("Unknown color or not a string: {}", color);
				}
			}
		}
	}
	game_id
}

pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file: File_content: String = String::new();


	file.read_to_string(&mut file_content).expect("Failed to read file content");
	let mut result = 0;
	let mut id: u32 = 1;
	for line in file_content.lines() {
		let modified_line = line.trim_start_matches(&format!("Game {}: ", id));

		result += evaluate_game(modified_line, id);
		id += 1;
    }

    result.to_string()
}