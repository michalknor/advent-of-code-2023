use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
<<<<<<< HEAD
	let mut file_content: String = String::new();
=======
	let mut file: File_content: String = String::new();
>>>>>>> 84da3640cab7e9c156fa7aa9bc01b57a9bcc4c39

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
	let data_enlargen: Vec<Vec<char>> = enlarge_rows_and_columns_with_empty_galaxies(data);
	let position_of_galaxies: Vec<[usize; 2]> = get_position_of_galaxies(&data_enlargen);
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


fn enlarge_rows_and_columns_with_empty_galaxies(
	data: &Vec<Vec<char>>
) -> Vec<Vec<char>> {
	let mut data_enlargen: Vec<Vec<char>> = data.to_owned();

	for i in (0..data.len()).rev() {
		let mut insert = true;

		for j in (0..data[0].len()).rev() {
			if data[i][j] == '#' {
				insert = false;
				break;
			}
		}

		if !insert {
			continue;
		}

		data_enlargen.insert(i, data[i].to_owned());
	}

	for j in (0..data[0].len()).rev() {
		let mut insert = true;

		for i in (0..data.len()).rev() {
			if data[i][j] == '#' {
				insert = false;
				break;
			}
		}

		if !insert {
			continue;
		}

		for i in 0..data_enlargen.len() {
			data_enlargen[i].insert(j, '.');
		}
	}

	data_enlargen
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