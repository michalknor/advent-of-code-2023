use std::collections::HashMap;

fn update_gears(gears: &mut HashMap<usize, Vec<u32>>, part_number: u32, adjacent_gears: &Vec<usize>) {
    for adjacent_gear in adjacent_gears {
        if !gears.contains_key(adjacent_gear) {
            gears.insert(*adjacent_gear, Vec::new());
        }
        gears.get_mut(adjacent_gear).unwrap().push(part_number);
    }
}


fn evaluate_schematic(schematic: Vec<Vec<char>>) -> u32 {
	let mut result: u32 = 0;
	let schematic_size = schematic.len();
	let mut gears: HashMap<usize, Vec<u32>> = HashMap::new();

	for (i, row) in schematic.iter().enumerate() {
		let mut part_number = 0;
		let mut adjacent_gears: Vec<usize> = Vec::new();
		
		for (j, character) in row.iter().enumerate() {
			let mut found_digit = false;

			if character.is_digit(10) {
				if part_number == 0 && j > 0 {
					if i > 0 && schematic.get(i-1).expect("REASON").get(j-1) == Some(&'*') {
						adjacent_gears.push((i-1) * schematic_size + j-1);
					}
					if schematic.get(i).expect("REASON").get(j-1) == Some(&'*') {
						adjacent_gears.push((i) * schematic_size + j-1);
					}
					if i < schematic_size - 1 && schematic.get(i+1).expect("REASON").get(j-1) == Some(&'*') {
						adjacent_gears.push((i+1) * schematic_size + j-1);
					}
				}
				
				part_number = part_number * 10 + character.to_digit(10).unwrap_or_default() as u32;
				found_digit = true;
			}

			if part_number == 0 {
				continue;
			}
			
			if i > 0 && schematic.get(i-1).expect("REASON").get(j) == Some(&'*') {
				adjacent_gears.push((i-1) * schematic_size + j);
			}
			
			if i < schematic_size - 1 && schematic.get(i+1).expect("REASON").get(j) == Some(&'*') {
				adjacent_gears.push((i+1) * schematic_size + j);
			}
			
			if !found_digit && schematic.get(i).expect("REASON").get(j) == Some(&'*') {
				adjacent_gears.push(i * schematic_size + j);
			}

			if found_digit {
				continue;
			}
		
			update_gears(&mut gears, part_number, &adjacent_gears);

			adjacent_gears = Vec::new();
			part_number = 0;
		}

		if part_number != 0 {
			update_gears(&mut gears, part_number, &adjacent_gears);
		}
    }

	for (_, value) in &gears {
		if value.len() == 2 {
			if let (Some(&first), Some(&second)) = (value.get(0), value.get(1)) {
				result += first * second;
			}
		}
    }

	result
}


pub fn main(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}
	
	let schematic: Vec<Vec<char>> = file_content.lines().map(|line| line.chars().collect()).collect();

	println!("{}", evaluate_schematic(schematic));
}