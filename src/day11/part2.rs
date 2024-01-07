use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::cmp;


const GALAXY_EXPANSION: usize = 1_000_000;


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file: File_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let data: Vec<Vec<char>> = file_content
		.lines()
		.map(|line| line
			.chars()
			.collect())
		.collect();

	sum_of_shortest_path_between_all_pairs(&data).to_string()
}


fn sum_of_shortest_path_between_all_pairs(
	data: &Vec<Vec<char>>
) -> usize {
	let position_of_empty_galaxies_x: HashSet<usize> = get_position_of_empty_galaxies_x(&data);
	let position_of_empty_galaxies_y: HashSet<usize> = get_position_of_empty_galaxies_y(&data);
	let position_of_galaxies: Vec<[usize; 2]> = get_position_of_galaxies(&data);
	let mut sum_of_shortest_path_between_all_pairs: usize = 0;

	for i in 0..position_of_galaxies.len() {
		for j in i+1..position_of_galaxies.len() {
			sum_of_shortest_path_between_all_pairs += 
				get_length_between_pair(
					[position_of_galaxies[i][0], position_of_galaxies[i][1]],
					[position_of_galaxies[j][0], position_of_galaxies[j][1]], 
					&position_of_empty_galaxies_x, 
					&position_of_empty_galaxies_y
				);
		}
	}

	sum_of_shortest_path_between_all_pairs
}


fn get_position_of_empty_galaxies_x(
	data: &Vec<Vec<char>>
) -> HashSet<usize> {
	let mut position_of_empty_galaxies_x: HashSet<usize> = HashSet::new();

	for i in 0..data.len() {
		let mut insert = true;

		for j in 0..data[0].len() {
			if data[i][j] == '#' {
				insert = false;
				break;
			}
		}

		if !insert {
			continue;
		}

		position_of_empty_galaxies_x.insert(i);
	}

	position_of_empty_galaxies_x
}


fn get_position_of_empty_galaxies_y(
	data: &Vec<Vec<char>>
) -> HashSet<usize> {
	let mut position_of_empty_galaxies_y: HashSet<usize> = HashSet::new();
	
	for j in 0..data[0].len() {
		let mut insert = true;

		for i in 0..data.len() {
			if data[i][j] == '#' {
				insert = false;
				break;
			}
		}

		if !insert {
			continue;
		}

		position_of_empty_galaxies_y.insert(j);
	}

	position_of_empty_galaxies_y
}


fn get_position_of_galaxies(
	data: &Vec<Vec<char>>
) -> Vec<[usize; 2]> {
	let mut position_of_galaxies: Vec<[usize; 2]> = Vec::new();

	for i in 0..data.len() {
		for j in 0..data[0].len() {
			if data[i][j] == '#' {
				position_of_galaxies.push([i, j]);
			}
		}
	}

	position_of_galaxies
}


fn get_length_between_pair(
	galaxy1: [usize; 2], 
	galaxy2: [usize; 2], 
	position_of_empty_galaxies_x: &HashSet<usize>, 
	position_of_empty_galaxies_y: &HashSet<usize>
) -> usize {
	
	galaxy1[0].abs_diff(galaxy2[0]) + galaxy1[1].abs_diff(galaxy2[1]) +
		get_number_of_empty_galaxies_between_pair(
			cmp::min(galaxy1[0], galaxy2[0]), 
			cmp::max(galaxy1[0], galaxy2[0]), 
			position_of_empty_galaxies_x
		) +
		get_number_of_empty_galaxies_between_pair(
			cmp::min(galaxy1[1], galaxy2[1]), 
			cmp::max(galaxy1[1], galaxy2[1]), 
			position_of_empty_galaxies_y
		)
}


fn get_number_of_empty_galaxies_between_pair(
	galaxy1: usize, 
	galaxy2: usize,
	position_of_empty_galaxies: &HashSet<usize>
) -> usize {
	let mut result: usize = 0;

	for position_of_empty_galaxy in position_of_empty_galaxies {
		if galaxy1 < *position_of_empty_galaxy && *position_of_empty_galaxy < galaxy2 {
			result += GALAXY_EXPANSION-1;
		}
	}

	result
}