use std::fs::File;
use std::io::Read;


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();

	file.read_to_string(&mut file_content).expect("Failed to read file content");

    let sequences: Vec<Vec<char>> = file_content
        .split(',')
        .map(|s| s.chars().collect())
        .collect();

    get_sum_of_sequences(&sequences).to_string()
}


fn get_sum_of_sequences(sequences: &Vec<Vec<char>>) -> usize {
    let mut sum: usize = 0;

    for sequence in sequences {
        let mut sequence_sum: usize = 0;
        for character in sequence {
            sequence_sum += *character as usize;
            sequence_sum *= 17;
            sequence_sum = sequence_sum % 256;
        }
        sum += sequence_sum
    }

    sum
}