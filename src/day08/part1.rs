use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

const STARTING_NODE: &str = "AAA";
const DESTINATION_NODE: &str = "ZZZ";


fn get_number_of_steps_to_reach_destination(
	network: HashMap<&str, Vec<&str>>, 
	instructions: &str,
	starting_node: &str
) -> u32 {
	if starting_node == DESTINATION_NODE {
		return 0;
	}

	let mut current_node = starting_node;
	let mut number_of_steps: u32 = 0;

	loop {
		for instruction in instructions.chars() {
			current_node = match instruction {
				'L' => network[current_node][0],
				'R' => network[current_node][1],
				_ => panic!("wrong instruction")
			};
			number_of_steps += 1;

			if current_node == DESTINATION_NODE {
				return number_of_steps;
			}
		}
	}
}


pub fn main(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open file");
	let mut file_content: String = String::new();


	file.read_to_string(&mut file_content).expect("Failed to read file content");

	let file_content = &(file_content.replace(['=', '(', ',', ')'], ""));
	
	let instructions: &str = file_content
		.lines()
		.next()
		.unwrap();

	let network: HashMap<&str, Vec<&str>> = file_content
		.split_whitespace()
		.skip(1)
		.collect::<Vec<_>>()
		.chunks_exact(3)
		.map(|chunk| (chunk[0], vec![chunk[1], chunk[2]]))
    	.collect::<HashMap<_, _>>();

	get_number_of_steps_to_reach_destination(network, instructions, STARTING_NODE).to_string()
}