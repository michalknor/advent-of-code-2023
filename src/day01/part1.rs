use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let mut result: u32 = 0;
	for line in file_content.lines() {
		let (mut first_digit, mut last_digit): (u32, u32) = (0, 0);

		for character in line.chars() {
			if character.is_ascii_digit() {
				first_digit = character.to_digit(10).unwrap_or_default();
				break;
			}
		}
		
		for character in line.chars().rev() {
			if character.is_ascii_digit() {
				last_digit = character.to_digit(10).unwrap_or_default();
				break;
			}
		}

		result += first_digit*10 + last_digit;
    }

    result.to_string()
}