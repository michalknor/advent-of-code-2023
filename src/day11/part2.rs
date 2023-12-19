use std::fs::File;
use std::io::Read;
use std::collections::HashSet;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

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
	data: &Vec<Vec<char>>,
	
) -> usize {
	let position_of_empty_galaxies_x: HashSet<usize> = get_position_of_empty_galaxies_x(&data);
	let position_of_empty_galaxies_y: HashSet<usize> = get_position_of_empty_galaxies_y(&data);
	let position_of_galaxies: Vec<[usize; 2]> = get_position_of_galaxies(&data);
	let mut sum_of_shortest_path_between_all_pairs: usize = 0;

	for i in 0..position_of_galaxies.len() {
		for j in i+1..position_of_galaxies.len() {
			sum_of_shortest_path_between_all_pairs += 
				position_of_galaxies[i][0].abs_diff(position_of_galaxies[j][0])
				+
				position_of_galaxies[i][1].abs_diff(position_of_galaxies[j][1]);
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