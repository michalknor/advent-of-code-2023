use std::fs::File;
use std::io::Read;


fn number_of_ways_to_win_a_race(time: u32, distance: u32) -> u32 {
	let mut result = 0;

	for i in 1..time-1 {
		if i * (time-i) > distance {
			result += 1;
		}
	}
	
	result
}


fn number_of_ways_multiplied_together(times: Vec<u32>, distances: Vec<u32>) -> u32 {
	let mut result = 1;

	for (i, time) in times.iter().enumerate() {
		result *= number_of_ways_to_win_a_race(*time, distances[i]);
	}

	result
}


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
<<<<<<< HEAD
	let mut file_content: String = String::new();
=======
	let mut file: File_content: String = String::new();
>>>>>>> 84da3640cab7e9c156fa7aa9bc01b57a9bcc4c39


	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let times: Vec<u32> = file_content
		.lines()
		.next()
		.unwrap()
		.split(' ')
		.skip(2)
		.filter(|&s| !s.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	let distances: Vec<u32> = file_content
		.lines()
		.nth(1)
		.unwrap()
		.split(' ')
		.skip(2)
		.filter(|&s| !s.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	number_of_ways_multiplied_together(times, distances).to_string()
}