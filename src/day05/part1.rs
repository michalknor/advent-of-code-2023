fn convert_seed_to_location(seed: u64, recipe: &Vec<Vec<u64>>) -> u64 {
	for conversion in recipe {
		let conversion_to = conversion[0];
		let conversion_from = conversion[1];
		let conversion_range = conversion[2];

		if conversion_from <= seed && seed < conversion_from + conversion_range {
			return conversion_to + seed - conversion_from;
		}
	}

	seed
}


pub fn main(testing: bool) -> String {
	let file_content: &str = if testing {
		include_str!("test.txt")
	}
	else {
		include_str!("input.txt")
	};
	
	let seeds: Vec<u64> = file_content.lines().next().unwrap().trim_start_matches("seeds: ").split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

	let mut recipes: Vec<Vec<Vec<u64>>> = Vec::new();
	
	for line in file_content.lines().skip(1) {
		if line.is_empty() {
			recipes.push(Vec::new());
			continue;
		}

		if line.contains(':') {
			continue;
		}

		recipes.last_mut().unwrap().push(
			line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect()
		);
	}

	let mut min_location: u64 = u64::MAX;

	for seed in seeds {
		let mut location = seed;

		for recipe in &recipes {
			location = convert_seed_to_location(location, recipe);
		}

		if min_location > location {
			min_location = location;
		}
	}

	min_location.to_string()
}