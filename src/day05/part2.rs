use std::sync::{Arc, Mutex};
use rayon::prelude::*;

fn seed_in_ranges_of_seed_numbers(seed: u64, ranges_of_seed_numbers: &Vec<u64>) -> bool {
	for i in 0..ranges_of_seed_numbers.len() / 2 {
        let seed_from = ranges_of_seed_numbers[2 * i];
        let seed_to = seed_from + ranges_of_seed_numbers[2 * i + 1];
		if seed_from <= seed && seed <= seed_to {
			return true;
		}
    }

	false
}


fn convert_location_to_seed(location: u64, recipe: &Vec<Vec<u64>>) -> u64 {
	for conversion in recipe {
		let conversion_to = conversion[0];
		let conversion_from = conversion[1];
		let conversion_range = conversion[2];

		if conversion_to <= location && location < conversion_to + conversion_range {
			return conversion_from + location - conversion_to;
		}
	}

	location
}


pub fn main(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}
	
	let ranges_of_seed_numbers: Vec<u64> = file_content.lines().next().unwrap().trim_start_matches("seeds: ").split(" ").map(|s| s.parse::<u64>().unwrap()).collect();

	let mut recipes: Vec<Vec<Vec<u64>>> = Vec::new();
	
	for line in file_content.lines().skip(1) {
		if line.is_empty() {
			recipes.push(Vec::new());
			continue;
		}

		if line.contains(":") {
			continue;
		}

		recipes.last_mut().unwrap().push(
			line.split(" ").map(|s| s.parse::<u64>().unwrap()).collect()
		);
	}

	let mut location = 0;

	let min_location = loop {
		let mut seed = location;

		for recipe in recipes.iter().rev() {
			seed = convert_location_to_seed(seed, recipe);
			if location == 1493866 {
				println!("{}, {}", location, seed);
			}
		}

		if seed_in_ranges_of_seed_numbers(seed, &ranges_of_seed_numbers) {
			println!("{}, {}", location, seed);
			break location;
		}

		location += 1;
	};

	println!("{}", min_location);
}

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

//brute force
pub fn main2(testing: bool) {
	let file_content: &str;
	if testing {
		file_content = include_str!("test.txt");
	}
	else {
		file_content = include_str!("input.txt");
	}
	
	let ranges_of_seed_numbers: Vec<u64> = file_content.lines().next().unwrap().trim_start_matches("seeds: ").split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

	println!("im here");

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

	let min_location = Arc::new(Mutex::new(u64::MAX));

	ranges_of_seed_numbers.par_chunks(2).for_each(|chunk| {
		let seed = chunk[0];
		let number_of_seeds = chunk[1];

		(seed..seed + number_of_seeds).into_par_iter().for_each(|j| {
			let mut location = j;

			for recipe in &recipes {
				location = convert_seed_to_location(location, recipe);
			}

			// Use a mutex to safely update min_location
			let mut min_location_guard = min_location.lock().unwrap();
			if *min_location_guard > location {
				*min_location_guard = location;
				println!("location {min_location_guard} found by seed {j}");
			}
		});
	});

	println!("{}", *min_location.lock().unwrap());
}