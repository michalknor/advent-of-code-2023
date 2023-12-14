use std::fs::File;
use std::io::Read;


const BASE_NUMBER: u32 = 2;


fn evaluate_ticket(winning_numbers: Vec<u32>, your_numbers: Vec<u32>) -> u32 {
	let mut result = 0;

	for winning_number in winning_numbers {
		if your_numbers.contains(&winning_number) {
            result += 1;
        }
	}
	if result == 0 {
		return 0;
	}
	BASE_NUMBER.pow(result-1)
}


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();


	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let mut result = 0;
	
	for line in file_content.lines() {
		let modified_line: Vec<String> = line.replace("  ", " ").split(": ").nth(1).unwrap_or("").split(" | ").map(String::from).collect();
		
		let winning_numbers: Vec<u32> = modified_line[0].split(' ').map(|s| s.parse::<u32>().expect("Failed to parse as u32")).collect();
		let your_numbers: Vec<u32> = modified_line[1].split(' ').map(|s| s.parse::<u32>().expect("Failed to parse as u32")).collect();

		result += evaluate_ticket(winning_numbers, your_numbers);
    }

    result.to_string()
}



//80694070 high