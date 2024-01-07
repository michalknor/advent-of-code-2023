use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

const STARTING_NODE: &str = "A";
const DESTINATION_NODE: &str = "Z";

pub fn main(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Failed to open file");
	let mut file: File_content: String = String::new();


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

	let starting_nodes: Vec<&str> = network
		.keys()
        .filter(|&&node| node.ends_with(STARTING_NODE))
		.cloned()
        .collect();

	get_number_of_steps_to_reach_destination(&network, instructions, &starting_nodes).to_string()
}


fn get_number_of_steps_to_reach_destination(
	network: &HashMap<&str, Vec<&str>>, 
	instructions: &str,
	starting_nodes: &Vec<&str>
) -> usize {
	let mut number_of_steps: Vec<usize> = starting_nodes
		.iter()
		.map(
			|current_node|
			process_instructions(network, instructions, current_node)
		)
		.collect();

	number_of_steps.sort();
	number_of_steps.reverse();

	least_common_multiple(&number_of_steps)
}


fn process_instructions(
    network: &HashMap<&str, Vec<&str>>,
    instructions: &str,
    starting_node: &str
) -> usize {
	let mut current_node: &str = starting_node;
    let mut number_of_steps: usize = 0;

	loop {
		for instruction in instructions.chars() {
			number_of_steps += 1;
			current_node = match instruction {
				'L' => network[current_node][0],
				'R' => network[current_node][1],
				_ => panic!("wrong instruction"),
			};

			if current_node.ends_with(DESTINATION_NODE) {
				return number_of_steps;
			}
		}
	}
}


fn least_common_multiple(number_of_steps: &Vec<usize>) -> usize {
    number_of_steps
		.iter()
        .fold(
			number_of_steps[0], 
			|result, &number| 
			result * number / greatest_common_divisor(number, result)
		)
}


fn greatest_common_divisor(smaller_number: usize, larger_number: usize) -> usize {
	(1..=smaller_number)
		.rev()
		.find(|&i| 
			smaller_number % i == 0 && larger_number % i == 0
		)
		.unwrap_or(1)
}