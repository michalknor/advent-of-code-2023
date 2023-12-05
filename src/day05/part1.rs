fn convert_seed_to_location(seed: u32, recipe: &Vec<Vec<u32>>) -> u32 {
	let mut location: u32 = seed;

	for conversion in recipe {
		let conversion_from = conversion[0];
		let conversion_to = conversion[1];
		let conversion_add = conversion[2];
		if conversion_from <= seed && seed <= conversion_to {
			return seed + conversion_add;
		}
	}

	seed
}


pub fn main(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}

	let mut seeds: Vec<u32> = file_content.lines().next().unwrap().trim_start_matches("seeds: ").split(" ").map(|s| s.parse::<u32>().unwrap()).collect();
	let mut locations: Vec<u32> = Vec::new();

	let mut recipes: Vec<Vec<Vec<u32>>> = Vec::new();
	
	for line in file_content.lines().skip(1) {
		if line == "" {
			recipes.push(Vec::new());
			continue;
		}

		if line.contains(":") {
			continue;
		}

		recipes.last_mut().unwrap().push(
			line.split(" ").map(|s| s.parse::<u32>().unwrap()).collect()
		);
    }

	for seed in seeds {
		let mut location = seed;
		for recipe in &recipes {
			location = convert_seed_to_location(seed, &recipe);
			break;
		}

		locations.push(location);
	}

	println!("{:?}", locations);

	println!("{}", locations.iter().min().unwrap());
}