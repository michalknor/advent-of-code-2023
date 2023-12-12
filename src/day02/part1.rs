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

pub fn main(testing: bool) -> String {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};
	let mut result = 0;
	let mut id: u32 = 1;
	for line in file_content.lines() {
		let modified_line = line.trim_start_matches(&format!("Game {}: ", id));

		result += evaluate_game(modified_line, id);
		id += 1;
    }

	println!("{}", result);

    result.to_string()
}