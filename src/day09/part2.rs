use std::fs::File;
use std::io::Read;


fn sum_extrapolated_values(histories: &Vec<Vec<i32>>) -> i32 {
	let mut sum_extrapolated_values: i32 = 0;

	for history in histories {
		let mut current_history: Vec<i32> = history.clone();
		current_history.reverse();

		loop {
			let mut all_zeroes = true;

			for i in 0..current_history.len() - 1 {
				current_history[i] = current_history[i+1] - current_history[i];
				if current_history[i] != 0 {
					all_zeroes = false;
				}
			}

			sum_extrapolated_values += current_history.pop().unwrap();

			if all_zeroes {
				break;
			}
		}
	}

	sum_extrapolated_values
}


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();


	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let histories: Vec<Vec<i32>> = file_content
		.lines()
		.map(|line| line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect())
		.collect();

	sum_extrapolated_values(&histories).to_string()
}