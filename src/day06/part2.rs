use std::fs::File;
use std::io::Read;


fn number_of_ways_to_win_a_race(time: u64, distance: u64) -> u64 {
	let mut result = 0;

	for i in 1..time-1 {
		if i * (time-i) > distance {
			result += 1;
		}
	}
	
	result
}


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file: File_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let times: u64 = file_content
		.lines()
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.collect::<String>()
		.parse()
    	.unwrap();

	let distance: u64 = file_content
		.lines()
		.nth(1)
		.unwrap()
		.split_whitespace()
		.skip(1)
		.collect::<String>()
		.parse()
    	.unwrap();

	number_of_ways_to_win_a_race(times, distance).to_string()
}