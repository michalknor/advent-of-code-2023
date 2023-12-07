const IGNORED_CHARACTER: char = '.';

fn is_symbol(character: &char) -> bool {
	if character.is_ascii_digit() || character == &IGNORED_CHARACTER {
		return false;
	}
	true
}


fn evaluate_schematic(schematic: Vec<Vec<char>>) -> u32 {
	let mut result: u32 = 0;
	let schematic_size = schematic.len();

	for (i, row) in schematic.iter().enumerate() {
		let mut adjacent_to_a_symbol = false;
		let mut part_number = 0;
		let mut found_digit = false;
		for (j, character) in row.iter().enumerate() {
			let mut adjacent_to_a_symbol_found_now = false;

			if character.is_ascii_digit() {
				part_number = part_number * 10 + character.to_digit(10).unwrap_or_default();
				found_digit = true;
			}
			else if !found_digit {
				adjacent_to_a_symbol = false;
			}
			else {
				found_digit = false;
			}

			if adjacent_to_a_symbol && found_digit {
				continue;
			}
			
			if (i > 0 && is_symbol(schematic.get(i-1).expect("REASON").get(j).unwrap())) 
			|| ((part_number == 0 || !found_digit) && *character != IGNORED_CHARACTER) 
			|| (i < schematic_size - 1 && is_symbol(schematic.get(i+1).expect("REASON").get(j).unwrap()))  {
				adjacent_to_a_symbol = true;
				adjacent_to_a_symbol_found_now = true;
			}

			if found_digit || part_number == 0 {
				continue;
			}

			if adjacent_to_a_symbol {
				result += part_number;
				adjacent_to_a_symbol = adjacent_to_a_symbol_found_now;
			}

			part_number = 0;
		}

		if part_number == 0 {
			continue;
		}
		
		if adjacent_to_a_symbol {
			result += part_number;
		}
    }

	result
}


pub fn main(testing: bool) {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};
	
	let schematic: Vec<Vec<char>> = file_content.lines().map(|line| line.chars().collect()).collect();

	println!("{}", evaluate_schematic(schematic));
}